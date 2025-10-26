/* Cria link simbólico tanto para a versão em debug, quanto para o binário 
 * final. */
#[cfg(target_os="linux")]
use std::os::unix::fs::symlink;
use std::env::current_exe;
use std::path::{PathBuf, Path};
use std::ffi::{OsStr};
use std::io::{ErrorKind};

/* Nota: se este código foi copiado prá algum outro projeto, diferente do 
 * original dele, a 'cobrinha-classica', provavelmente você precisa mudar 
 * a constante literal 'string', com o nome do projeto original, prá o nome
 * do diretório que está o código do projeto. */
const PROJETO: &'static str = "alternador-wallpapers";


/// Complementa link ao executável à partir do caminho do executável ...
#[allow(unused)]
pub fn computa_caminho(caminho_str: &str) -> PathBuf {
   const NOME: &'static str = PROJETO;
   let barreira = Some(OsStr::new(NOME));

   /* O método novo, busca algo mais flexível. Ele pode capturar o caminho
    * do projeto baseado em qualquer profundidade dentro dele. */
   match current_exe() {
      Ok(mut executavel) => {
         while executavel.file_name() != barreira {
            executavel.pop();
         }
         executavel.push(caminho_str);
         executavel
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

#[allow(unused)]
#[cfg(target_os="linux")]
pub fn linka_executaveis(nome: &str) {
   // caminho aos executáveis.
   let caminho_str = "target/release/cobrinha_classica";
   let executavel = computa_caminho(caminho_str);
   let caminho_str = "target/debug/cobrinha_classica";
   let executavel_debug: PathBuf = computa_caminho(caminho_str);

   // seus links simbólicos:
   let ld_link = computa_caminho(nome);
   let mut nome_debug = nome.to_string();
   nome_debug.push_str("_debug");
   let ld_debug_link = computa_caminho(nome_debug.as_str());

   if ld_link.as_path().exists() && 
   ld_link.as_path().is_symlink() {
      if executavel.as_path().exists() 
         { println!("binário do executável existe."); }
   } else {
      print!("criando '{}' ... ", nome);
      match symlink(executavel.as_path(), ld_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }

   if ld_debug_link.as_path().exists() && 
   ld_link.as_path().is_symlink() { 
      if executavel_debug.exists() 
         { println!("binário do executável(DEBUG) existe."); }
   } else {
      print!("criando '{}'(debug) ... ", nome_debug);
      match symlink(executavel_debug.as_path(), ld_debug_link.as_path()) {
         Ok(_) => {
            println!("com sucesso.");
         } Err(_) => 
            { println!("executável não existe!"); }
      };
   }
}

use std::fs::{remove_file};
#[cfg(target_os="linux")]
pub fn linca_executaveis_externamente(nome: &str) -> 
  Result<PathBuf, std::io::ErrorKind> 
{
   // caminho aos executáveis.
   let executavel = current_exe().unwrap();
   // destino do linque agora é no global, se houver um é claro.
   let destino: &'static str = env!("LINKS");
   let linque = {
      let nome_dbg = format!("{}-debug", nome);

      if cfg!(debug_assertions) 
         { Path::new(destino).join(nome_dbg) }
      else
         { Path::new(destino).join(nome) }
   };
   let ja_existe_um_linque_simbolico = {
      linque.exists() && 
      linque.is_symlink()
   };

   if cfg!(debug_assertions) { 
      println!("resultado do link='{}'", linque.display()); 
      println!("existência? {}", ja_existe_um_linque_simbolico);
   }
      
   if ja_existe_um_linque_simbolico {
      println!("binário do executável já existe em {}.", destino); 
      // apenas retorna o linque que já existe!
      return Ok (linque);
   } 

   print!("Criando '{}' ... ", nome);
   match symlink(executavel.as_path(), &linque) {
      Ok(_) => {
         println!("com sucesso.");
         // Apenas retorna caminho do linque criado.
         Ok (linque)
      } Err(tipo_de_erro) => { 
         match tipo_de_erro.kind() {
            ErrorKind::AlreadyExists => {
               print!("Excluindo {} ...", linque.display());
               remove_file(&linque).unwrap();
               println!("feito!");
               Err(ErrorKind::TooManyLinks)
            } _=> 
               { Err(tipo_de_erro.kind()) }
         }
      }   
   }
}

