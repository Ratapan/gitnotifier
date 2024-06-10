use std::path::Path;
use std::process::Command;

#[tauri::command]
pub fn check_git_status(repos: Vec<String>) -> Result<String, String> {
    for repo in repos {
        if Path::new(&format!("{}/.git", repo)).exists() {
            let output = Command::new("git")
                .arg("-C")
                .arg(&repo)
                .arg("status")
                .arg("--porcelain")
                .output()
                .map_err(|e| format!("Error ejecutando git: {}", e))?;

            if !output.stdout.is_empty() {
                return Err(format!("Cambios no guardados en: {}", repo));
            }
        } else {
            return Err(format!("No es un repositorio git: {}", repo));
        }
    }
    Ok("Todos los cambios están subidos.".into())
}