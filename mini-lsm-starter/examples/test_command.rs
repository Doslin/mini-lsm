use std::io::{self, Write};
use std::process::Command;
fn main() -> io::Result<()> {
    let url = "kmtrigger://macro=671BF7D4-3CBA-4E19-9EBB-897B22435C67&value=O-%E6%8C%81%E7%BB%AD%E6%8E%A8%E8%BF%9B%20Pandora%20%E5%B9%B3%E5%8F%B0%E7%9A%84%E5%90%84%E9%A1%B9%E4%B8%9A%E5%8A%A1%E6%94%AF%E6%8C%81%E5%B7%A5%E4%BD%9C-2023-Q1
nats%20metrics%20%E6%90%AD%E5%BB%BA%E5%92%8C%E6%8E%A5%E5%85%A5%E5%88%B0%20grafana";
    Command::new("open").arg(url).spawn()?.wait()?;
    writeln!(io::stdout(), "Command executed successfully")?;
    Ok(())
}
