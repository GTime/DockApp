use std::fs::read_dir;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

// Get list of all file in the composes folder
fn get_composers(path: &str) -> io::Result<Vec<PathBuf>> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}

// Present File file to the user
fn prepare_menu(composers: &Vec<PathBuf>) -> String {
    let mut counter = 1;
    let mut menu = String::new();
    for entry in composers {
        menu.push_str(&format!(
            "{}: {}\n",
            counter,
            entry.file_stem().unwrap().to_str().unwrap()
        ));
        counter += 1;
    }
    menu
}
// Take the user's choice
fn get_input(question: &str) -> String {
    let mut buf = String::new();
    print!("{}", question);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buf).expect("stdin() failed");
    buf.trim().to_string()
}

// Run docker-compose on the user's choice
fn compose(path: &PathBuf) -> io::Result<()> {
    loop {
        // systemctl status docker
        let check_docker = Command::new("sudo")
            .args(&["systemctl", "status", "docker"])
            .output()
            .expect("Failed to execute command");

        if !check_docker.status.success() {
            // systemctl restart docker
            Command::new("sudo")
                .args(&["systemctl", "restart", "docker"])
                .output()
                .expect("Failed to execute command");
        } else {
            println!("{}", path.to_str().unwrap());

            // docker-compose -f $HOME"path" up
            Command::new("sudo")
                .args(&["docker-compose", "-f", path.to_str().unwrap(), "up"])
                .spawn()
                .expect("Failed to execute command");

            break;
        }
    }

    Ok(())
}

const COMPOSERS_PATH: &str = "./composers";

fn main() -> io::Result<()> {
    println!("\nHi, Welcome\n");

    // Getting Composers
    let composers = get_composers(COMPOSERS_PATH)?;
    println!("Select from the list of composers: ");
    println!("{}", prepare_menu(&composers));

    loop {
        let user_input = get_input("Which do you want to compose? ");
        if user_input == ":quit" {
            println!("Goodbye!");
            break;
        }

        let choice = user_input.parse::<usize>().unwrap();
        if choice > 0 && choice <= composers.len() {
            println!("");
            compose(&composers[choice - 1])?;
            break;
        } else {
            println!("\nInvalid Option!");
            continue;
        }
    }

    Ok(())
}

// TEST
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_composers() {
        let composers = get_composers(COMPOSERS_PATH);
        assert!(composers.is_ok());
        assert!(!composers.unwrap().is_empty());
    }

    #[test]
    fn test_get_input() {
        let user_input = get_input("This is a test, what do you say? ");
        assert!(user_input == "cool");
    }

    #[test]
    fn test_compose() {
        let composer = PathBuf::from(COMPOSERS_PATH);
        assert!(compose(&composer).is_ok())
    }

    #[test]
    fn test_prepare_menu() {
        let composers = get_composers(COMPOSERS_PATH).unwrap();
        assert_eq!(prepare_menu(&composers), "");
    }
}
