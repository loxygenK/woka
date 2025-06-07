use std::process::Command;

pub fn construct_ssh_cmd(hostname: &str) -> Command {
    let mut cmd = Command::new("ssh");
    cmd.arg("-o").arg("ConnectTimeout=10")
       .arg("-o").arg("BatchMode=no")
       .arg(hostname);

    cmd
}

