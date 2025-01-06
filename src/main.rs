use std::process::Command;

fn check_if_files_exists() {
    
}

fn download_data() {
    let baseurl = "https://arquivos.receitafederal.gov.br/dados/cnpj/dados_abertos_cnpj/2024-12/";
    for n in 0..10 {
        let url = format!("{}Estabelecimentos{}{}", baseurl, n, ".zip");
        if let Some(file_name) = url.split('/').last() {
            let mut process = Command::new("aria2c")
                .args(["-o", &file_name, &url])
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
}

fn main() {
    download_data();
}
