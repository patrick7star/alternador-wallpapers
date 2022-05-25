
/** 
 BD para gravar em disco todas alterações
 já realizadas, como também para ajudar 
 na reparações de redundâncias feitas na
 seleção aleatória.
*/


// biblioteca padrão do Rust:
use std::fs::{OpenOptions, read_to_string, File};
use std::io::{Write};
use std::path::{Path, PathBuf};
// usando própria biblioteca:
use crate::BD1;


pub fn grava_escolha(caminho:PathBuf) -> bool {
   // abrindo bd ...
   let mut arquivo:File = {
      OpenOptions::new()
      .create(true)
      .write(true)
      .open(BD1)
      .unwrap()
   };
   // todas modificações feitas.
   let nome:&str = {
      caminho
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
   };
   let diretorio:&str = {
      caminho.as_path()
      .parent().unwrap()
      .to_str().unwrap()
   };
   // primeiro o caminho até o arquivo.
   arquivo.write(diretorio.as_bytes()).unwrap();
   // quebra-de-linha necessária para separação.
   arquivo.write(b"\n").unwrap();
   // agora o nome do arquivo em sí.
   arquivo.write(nome.as_bytes()).unwrap();
   // quebra-de-linha necessária para separação.
   arquivo.write(b"\n").unwrap();
   // confirmação de tudo okay ...
   return true;
}

pub fn le_escolha() -> Result<PathBuf, &'static str> {
   // lendo todo arquivo, e colocando num interador
   // baseado nas quebra-de-linhas.
   let conteudo:String = {
      let pth = Path::new(BD1);
      match read_to_string(pth) {
         Ok(resultado) => resultado,
         Err(_) => { return Err("arquivo foi apagado!"); }
      }
   };
   let mut conteudo = conteudo.lines();
   let diretorio = conteudo.next().unwrap();
   let nome = conteudo.next().unwrap();
   // formando 'Caminho(Path)' ...
   let mut caminho:PathBuf = PathBuf::new();
   caminho.push(diretorio);
   caminho.push(nome);
   // retornando o que foi obtido.
   return Ok(caminho);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn testando_escrita() {
      let arquivo_teste = Path::new("dir/sub_dir/sub_sub_dir/arquivo.txt")
      .to_path_buf();
      assert!(grava_escolha(arquivo_teste.clone()));
   }

   #[test]
   fn testa_leitura_dos_dados() {
      let caminho = le_escolha().unwrap();
      let mut caminho_i = PathBuf::new();
      caminho_i.push("dir");
      caminho_i.push("sub_dir");
      caminho_i.push("sub_sub_dir");
      caminho_i.push("arquivo.txt");
      assert_eq!(caminho, caminho_i);
   }
}
