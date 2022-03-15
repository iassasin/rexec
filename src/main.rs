mod config;

use std::{vec, io::{Cursor, Write, Read}, collections::HashMap, process::{Command, Stdio}};

use config::{AppConfig, TaskConfig};
use tiny_http::{Server, Response, Request};

fn main() {
    let config = AppConfig::read_from_file("./rexec.yml");

    let listen_address = format!("{}:{}", config.http.listen_ip, config.http.port);
    println!("Listen to address: {:?}", listen_address);

    let server = Server::http((config.http.listen_ip, config.http.port)).unwrap();

    for mut request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let response = match process_request(&mut request, &config.tasks) {
            Ok(response) => response,
            Err(err) => Response::from_string(err).with_status_code(500),
        };

        request.respond(response).unwrap_or_default();
    }
}

fn process_request(request: &mut Request, tasks: &HashMap<String, TaskConfig>) -> Result<Response<Cursor<Vec<u8>>>, String> {
    const TASKS_PATH: &str = "/task/";

    let url = request.url().to_string();
    if !url.starts_with(TASKS_PATH) {
        return Ok(Response::from_string("Path not found").with_status_code(404));
    }

    let (_, task_name) = url.split_at(TASKS_PATH.len());

    if let Some(task_config) = tasks.get(task_name) {
        let mut buf = vec![];
        let _size = request.as_reader()
            .read_to_end(&mut buf)
            .map_err(|err| "Bad body reader: ".to_string() + &err.to_string())?;

        let mut command = Command::new("sh")
            .arg("-c")
            .arg(task_config.command.clone() + " 2>&1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| "Can't spawn process: ".to_string() + &err.to_string())?;

        command.stdin.as_ref()
            .ok_or("Can't write body to process: no stdin")?
            .write_all(&buf)
            .map_err(|err| "Can't write body to process: ".to_string() + &err.to_string())?;

        let mut outbuf = vec![];
        let _outsize = command.stdout.as_mut()
            .ok_or("Can't grab process output. But maybe it completed?")?
            .read_to_end(&mut outbuf)
            .map_err(|err| "Bad stdout reader: ".to_string() + &err.to_string())?;

        let _status = command.wait()
            .map_err(|err| "Bad process exit: ".to_string() + &err.to_string())?;

        return Ok(Response::from_data(outbuf));
    }

    Ok(Response::from_string(format!("Unknown task: {:?}", task_name)).with_status_code(404))
}
