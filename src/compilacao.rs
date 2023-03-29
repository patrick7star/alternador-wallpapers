

/** 
 Executa compilação do artefato otimizado
 na iniciação do programa, porém, o faz em
 paralelo com a execução, já que pode parar
 o programa se houver algum erro, ou interromper
 a execução do restante do código por um
 bom tempo.
*/

use std::{env, ffi::{OsStr}};
use std::path::{Path, Component, PathBuf};
use std::process::Command;
/* compila binário principal se, primeiro,
 * não estiver rodando ele; outra é que,
 * também ele não existir. Faz isso por
 * um subprocesso, para que não interfira
 * na chamada da função principal, nem
 * interrompa o programa em caso de error. */
fn hora_de_compilar() -> bool {
   let rodando_no_debug: bool = {
      match dbg!(env::current_exe()) {
         Ok(caminho) => {
            let str = OsStr::new("debug");
            let m = Component::Normal(str);
            caminho.components().any(|c| c == m)
         } Err(_) => false
      }
   };
   let traducao = { |logico| 
      if logico
         { String::from("verdadeiro") }
      else
         { String::from("falso") }
   };
   let s = "target/release/alternador_wallpapers";
   let caminho_otimizado = dbg!(computa_caminho(s));

   /* impressão das condições necessárias, 
    * para que se autorize a compilação. */
   println!(
      "rodando versão de debug: {}
      \rexecutável otimizado existe? {}", 
      traducao(rodando_no_debug),
      traducao(caminho_otimizado.exists())
   );
   rodando_no_debug && !caminho_otimizado.exists()
}

// complementa link ao executável.
fn computa_caminho<C>(caminho: C) -> PathBuf
  where C: AsRef<Path> 
{
   // à partir do caminho do executável ...
   match env::current_exe() {
      Ok(mut base) => {
         while !base.ends_with("target")
            { base.pop(); }
         /* remove também o diretório 'target'.
          * Então agora está no diretório
          * do 'caixote' do código. */
         base.pop(); 
         // complementa com o caminho passado.
         base.push(caminho);
         match base.canonicalize() {
            Ok(path) => path,
            Err(_) => base
         }
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

/* executa compilação em sí. Confirma
 * se foi feito, ou não. */
pub fn executa_compilacao() -> bool {
   // caminho do caixote.
   let diretorio = computa_caminho(".");
   // compondo o comando...
   let mut comando = Command::new("cargo");
   comando.arg("build");
   comando.arg("--release");
   comando.arg("--offline");
   comando.arg("--color=never");
   comando.current_dir(diretorio);

   if hora_de_compilar() {
      match comando.spawn() {
         Ok(_) => {
            print!("rodando compilação...");
         } Err(_) => { return false; }
      };
      println!("feito.");
      return true;
   } else { false }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn HoraDeCompilarSaidas() 
      { println!("resultado: {}", hora_de_compilar()); }

   #[test]
   fn diretorioRaizDoCaixote() {
      let c = dbg!(computa_caminho("."));
      assert!(c.exists());
      let cII = dbg!(env::current_dir().unwrap());
      assert_eq!(c.clone(), cII.clone());
      let cIII = Path::new("/etc").to_path_buf();
      env::set_current_dir(dbg!(cIII.clone())).unwrap();
      assert_eq!(cIII, env::current_dir().unwrap());
      assert_eq!(c, cII);
   }

   #[test]
   fn executaCompilacao() {
      executa_compilacao();
   }
}
