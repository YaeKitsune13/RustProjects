use tokio::process::{Child, Command};

#[tokio::main]
async fn main() {
    let mut comfy_process = start_comfy().await;
    println!("ComfyUI запущен! PID процесса: {:?}", comfy_process.id());

    let status = comfy_process
        .wait()
        .await
        .expect("Ошибка во время работы ComfyUI");
    println!("ComfyUI завершился со статусом: {}", status);
}

async fn start_comfy() -> Child {
    Command::new("/home/yaekitsune/Projects/ComfyUI/.venv/bin/python")
        .current_dir("/home/yaekitsune/Projects/ComfyUI")
        .args([
            "main.py",
            "--enable-manager",
            "--enable-manager-legacy-ui",
            "--preview-method",
            "auto",
        ])
        .kill_on_drop(true)
        .spawn()
        .expect("Не удалось запустить ComfyUI")
}
