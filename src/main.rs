use std::process::Command;
use std::fs;

fn check_if_files_exist() -> Vec<String> {
    let mut no_existing_files: Vec<String> = Vec::new();
    let base_path = "/home/ubuntu/Documents/projetos/cnpj_to_sqlite/downloads/estabelecimentos/";
    for i in 0..10 {
        let file_name = format!("{}Estabelecimentos{}.zip", base_path, i);
        if fs::metadata(&file_name).is_ok() {
            println!("File {} exists, skipping", &file_name);
        } else {
            println!("File {} does not existing, added to no_existing_files!", &file_name);
            no_existing_files.push(file_name);
        }
    }
    no_existing_files
}

fn download_data(missing_files: Vec<String>) {
    let baseurl = "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/2024-12/";
    for file_name in missing_files {
        let url = format!("{}{}", baseurl, file_name);
        let mut process = Command::new("aria2c")
            .args(["-o", ";/home/ubuntu/Documents/cnpj_to_sqlite/downloads/estabelecimentos/", &file_name, &url])
            .spawn()
            .expect("Falha ao executar o comando");

        let status = process.wait().expect("Falha ao esperar o processo!");

        if status.success() {
            println!("Download concluído para {}", file_name);
        } else {
            eprintln!(
                "Falha no download para o arquivo {}, código de saída: {:?}",
                file_name,
                status.code()
            );
        }
    }
}

fn main() {
    let missing_files = check_if_files_exist();
    download_data(missing_files);
}
