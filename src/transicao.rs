
// biblioteca padrão do Rust:
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::fs::{ReadDir};
use std::ffi::{OsString, OsStr};
use std::time::Duration;
use std::env;

// própria lib.
use super::RAIZ;
use super::banco_de_dados::{grava_escolha, le_escolha};

// bibliotecas externas:
extern crate xshell;
use xshell::{Cmd, cmd};
extern crate date_time;
pub use date_time::date_tuple::DateTuple;

/* acha todos XML que contém uma transição 
 * programada de determinados wallpapers, 
 * geralmente localizados no mesmo diretório 
 * de tal. */
pub fn parte_i() -> Vec<PathBuf> {
   // todos arquivos e subdiretórios da RAIZ dada.
   let sua_localizacao = {
      Path::new(RAIZ)
      .read_dir()
      .unwrap()
   };
   // lista com todos arquivos XML encontrados.
   let mut arquivos_xml:Vec<PathBuf> = Vec::new();
   // adicionando XML's do sistema.
   let caminhos:[&str; 3] = [
      "/usr/share/backgrounds/cosmos/background-1.xml",
      "/usr/share/backgrounds/contest/focal.xml",
      "/usr/share/backgrounds/ubuntu-mate-photos/ubuntu_mate_photos.xml"
   ];
   /* para adicionar caminho específico, 
    * adiciona na array. */
   for pth_str in caminhos.into_iter() {
      let caminho = Path::new(pth_str).to_path_buf();
      // só adiciona se existir.
      if caminho.exists() 
         { arquivos_xml.push(caminho); }
   }

   // varrendo tal raíz.
   for dir in sua_localizacao {
      // obtendo velho.
      let entrada = dir.unwrap().path();
      let diretorio = entrada.as_path();
      /* se for um diretório, entra nele e varre-o
       * por um padrão, que é: verificar se há arquivos
       * de imagens e um xml com o nome do diretório. */
      if diretorio.is_dir() {
         // novo iterador de entradas do subdiretório.
         let novas_dir:ReadDir = {
            diretorio
            .read_dir() 
            .unwrap()
         };
         for subdir in novas_dir {
            let entrada = subdir.unwrap().path();
            let caminho_i = entrada.as_path();
            let extensao = match caminho_i.extension() {
               Some(string) => string,
               None => OsStr::new("nada"),
            };
            //if caminho_i.extension().unwrap() == OsStr::new("xml"){ 
            if extensao == OsStr::new("xml") {
               arquivos_xml.push(caminho_i.to_path_buf());
               break;
            }
         }
      }
   }

   // retorna todos XML's encontrados.
   return arquivos_xml;
}

/* Seleciona uma nova transição de wallpapers
 * dado o dia. Ele busca antes uma lista quais 
 * foram as transições selecionadas no passado
 * para que, de modo randomico não haja repetiação.
 * O retorno é um caminho('PathBuf') extraido
 * provavelmente da array que os filtra.
*/
fn parte_ii() -> PathBuf {
   // todas transições-de-wallpapers conf.
   let mut todos = parte_i();
   // tem que ser não vázio.
   //assert!(todos.len() > 0);
   if todos.is_empty()
      { panic!("nenhum arquivo XML de transição foi encontrado!"); }
   // embalharando dados extraídos.
   fastrand::shuffle(&mut todos[..]);
   // qual indexar na array.
   let numero_sorteado = fastrand::usize(0..todos.len());
   // retornando seleção.
   return todos[numero_sorteado].clone();
}

/* 
 * Faz uma seleção levando transições de 
 * datas especiais em consideração na seleção.
 * Usa a função acima em consideração na 
 * seleção randômica.
*/
fn parte_iii(hoje:DateTuple) -> PathBuf {
   // obtem uma transição antes.
   let transicao = parte_ii();
   /* trabalha, inicialmente, em dois casos
    * especiais: Halloween e Natal.  */
   let mes = hoje.get_month();
   let dia = hoje.get_date();
   // Halloween ou próximo dele.
   let e_periodo_de_halloween:bool = {
      (dia >= 28 && mes == 9) ||
      (dia >= 1 && dia <= 31 && mes == 10) 
   };
   // Natal ou próximo dele.
   let e_periodo_de_natal:bool = {
      mes == 12 && dia >= 1 && dia <= 25
   };
   // Aniversário de Brasília.
   let e_aniversario_de_brasilia:bool = {
      mes == 4 && dia >= 12 && dia <= 21
   };

   // adicionando raíz ...
   let mut caminho:PathBuf = PathBuf::new();
   caminho.push(RAIZ);
   if e_periodo_de_halloween { 
      caminho.push("halloween");
      caminho.push("halloween.xml");
      return caminho;
   } else if e_periodo_de_natal { 
      caminho.push("natal");
      caminho.push("natal.xml");
      return caminho;
   } else if e_aniversario_de_brasilia { 
      caminho.push("brasília_wallpapers");
      caminho.push("brasília_wallpapers.xml");
      return caminho;
   }
   // todos demais caem neste caso.
   else { 
      let mut nova = transicao;
      let mut nome:&str = {
         nova.as_path()
         .file_name()
         .unwrap()
         .to_str()
         .unwrap()
      };
      let exclusao:[&'static str; 3]; 
      exclusao = [
         "natal.xml", 
         "halloween.xml", 
         "brasília_wallpapers.xml"
      ];
      while nome == "natal.xml" || 
      nome == "halloween.xml" ||
      nome == exclusao[2] 
      { 
         nova = parte_ii();
         nome = {
            nova.as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
         };
      }
      return nova; 
   }
}

/* tentanto reduzir repetições seguidas na 
 * seleção aleatória.
 */
fn parte_iv(hoje:DateTuple) -> PathBuf {
   let mut nova_transicao = parte_iii(hoje.clone());
   // o que foi selecionado anterior.
   match le_escolha() {
      Ok(selecao_anterior) => {
         // não pode ser igual para não causar eventos repetidos.
         while selecao_anterior == nova_transicao {
            /* caso seja algum dos períodos comemorativos
             * quebrar o laço, e seguir em frente. */
            let nome_arquivo = {
               selecao_anterior
               .as_path()
               .file_name()
               .unwrap()
            };
            let periodos_de_excecoes:bool = {
               nome_arquivo == "natal.xml" ||
               nome_arquivo == "halloween.xml" ||
               nome_arquivo == "brasília_wallpapers.xml"
            };
            if periodos_de_excecoes { break; }
            nova_transicao = parte_iii(hoje.clone());
         }
      },
      Err(erro) => 
         { println!("\nERROR:{} ... continuando mesmo assim", erro); }
   };
   // grava opção a retornar.
   grava_escolha(nova_transicao.clone());
   return nova_transicao;
}

/* ajusta encaixe da melhor, baseado na 
 * transição que foi escolhida.
 */
fn parte_v(caminho:PathBuf) -> PathBuf {
   let caminho_i:&Path = caminho.as_path();
   let nome:&OsStr =  caminho_i.file_name().unwrap();

   // ambiente que está inserido.
   let ambiente:&str = match env::var("XDG_CURRENT_DESKTOP") {
      Ok(a) => {
         if a == "ubuntu:GNOME"
            { "org.gnome.desktop.background" }
         else if a == "MATE"
            { "org.mate.background" }
         else
            { panic!("não implementado para tal ambiente!"); }
      },
      Err(_) => 
         { panic!("não implementado para tal ambiente!"); }
   };

   // melhor opção baseado na transição.
   let opcao:&str;
   if nome == OsStr::new("food_wallpapers.xml")
      { opcao = "zoom"; }
   else if nome == OsStr::new("brasília_wallpapers.xml")
      { opcao = "zoom"; }
   else if nome == OsStr::new("sombrio_wallpapers.xml")
      { opcao = "zoom"; }
   else 
      { opcao = "stretched"; }

   // criando comando.
   let comando:Cmd = {
      Cmd::new("gsettings")
      .args([
         "set", ambiente, 
         "picture-options", 
         opcao
      ].into_iter())
      .echo_cmd(false)
   };

   // rodando comando que troca opção da imagem ...
   comando.run().unwrap();

   // retornando caminho obtido ...
   return caminho;
}

/** executa o comando de troca de wallpapers
 acionando uma transição-de-wallpapers já 
 pré-configurada. */
pub fn alterna_transicao() {
   /* obtendo, de maneira aleatória, uma 
    * nova transição-de-wallpapers, baseado
    * na data de hoje. */
   let data_de_hoje = DateTuple::today();
   // último alteração foi ... :
   let ultima_tw:String = match le_escolha() {
      Ok(caminho) => {
         let str:String = String::from( 
            caminho.as_path()
            .file_name().unwrap()
            .to_str().unwrap()
         );
         // retornando string gerada acima ...
         str
      },
      Err(_) => String::from("nenhuma alteração anterior"),
   };
   let nova_transicao = parte_v(parte_iv(data_de_hoje));
   // arquivo selecionado agora:
   let atual_tw:&str = {
      nova_transicao.as_path()
      .file_name().unwrap()
      .to_str().unwrap()
   };
   // extraindo slice-string do 'PathBuf'.
   let caminho:&str = {
      nova_transicao
      .as_os_str()
      .to_str()
      .unwrap()
   };
   // iterador contendo todos argumentos.
   let (chave, atributo):(&'static str, &'static str) = {
      let ambiente = env::var("XDG_CURRENT_DESKTOP").unwrap(); 
      if ambiente == "ubuntu:GNOME"
         { ("org.gnome.desktop.background", "picture-uri") }
      else if ambiente == "MATE"
         { ("org.mate.background", "picture-filename") }
      else
         { panic!("não implementado para tal ambiente!"); }
   };
   let argumentos = ["set", chave, atributo, caminho].into_iter();

   // compondo todo comando.
   // seus argumentos necessários.
   // também desabilita escrita do comando no terminal.
   let comando:Cmd = {
      Cmd::new("gsettings")
      .args(argumentos)
      .echo_cmd(false)
   };
   // enfim, rodando o comando ...
   comando.run().unwrap();

   // mensagem informando o que está ocorrendo.
   println!(
      "\nalternância transição-de-wallpapers automaticamente acionada.
      \rseleção anterior: \"{}\"
      \rarquivo selecionado:\"{}\"",
      ultima_tw, atual_tw
   );
}

/* pega tempo da atual transição em segundos
 * e retorna um 'Duration'. */
pub fn duracao_atual_transicao() -> Duration {
   // pega dos registros último background trocado.
   let escolha:OsString = {
      le_escolha()
      .unwrap()
      .into_os_string()
   };
   // caminho até o script que pega dados do XML.
   let caminho_script = concat!(
      env!("RUST_CODES"),
      "/personalização/",
      "extern_lib/slide_background/xml_info.py"
   );
   let comando = cmd!("/usr/bin/python3 -O {caminho_script} {escolha}");
   // array com bytes do resultado!
   let mut resultado = comando.output().unwrap();
   // removendo quebra-de-linha ...
   resultado.stdout.pop().unwrap();
   // formando string com bytes representando número decimal.
   let mut decimal = String::new();
   for byte in resultado.stdout.iter() 
      { decimal.push(*byte as char); }
   
   // convertendo para um decimal ponto-flutuante ...
   match f32::from_str(decimal.as_str()) {
      // criando 'Duration' com valor recuperado ...
      Ok(segundos) => Duration::from_secs_f32(segundos),
      /* em caso de erro, sorteio um perído
       * entre uma hora à quatro horas. */
      Err(_) => {
         let limite_inferior: u64 = 3600;
         let limite_superior: u64 = 4 * 3600;
         let tempo = fastrand::u64(limite_inferior..limite_superior);
         Duration::from_secs(tempo) 
      }
   }
}

// testes realizados.
#[cfg(test)]
mod tests {
   use super::*;
   use std::fs::remove_file;
   extern crate utilitarios;
   use utilitarios::legivel::tempo;
   use crate::BD1;

   #[test]
   fn verifica_xmls_filtrados() {
      let xmls = parte_i();
      assert!(xmls.len() > 0);
      for arq in xmls.iter() 
         { println!("{:#?}", arq.as_path().file_name().unwrap()); }
      assert!(true);
   }

   #[test]
   fn selecao_de_transicao_aleatoria_amostra() {
      for ordem in 1..=100 {
         // nova transição selecionada aleatóriamente.
         let arquivo = parte_ii();
         println!(
            "{:3.0}ª seleção do arquivo: {:#?}", 
            ordem,
            arquivo.as_path()
            .file_name()
            .unwrap()
         );
      }
      assert!(true);
   }

   #[test]
   fn selecao_baseado_em_datas_comemorativas() {
      let mut inicio = DateTuple::new(1983, 3, 1).unwrap();
      for _ in 1..330 {
         // obtendo nova transição.
         let nt = parte_iii(inicio.clone());
         println!(
            "data: {}\nseleção: {:#?}\n",
            inicio.to_readable_string(),
            nt.file_name().unwrap()
         );
         // avançando dia ...
         inicio = inicio.next_date();
      }
      // conseguir atingir o que queria?
      assert!(true);
   }

   #[test]
   fn tenta_reduzir_repeticoes_seguidas() {
      // deletando banco de dados para não misturar as coisas.
      match remove_file(BD1) {
         Ok(_) => { println!("arquivo excluido com sucesso."); }
         Err(_) => { println!("o arquivo não existe!"); }
      };
      // data aleatória para teste de quase um ano.
      let mut inicio = DateTuple::new(1983, 3, 1).unwrap();
      for _ in 1..330 {
         // obtendo nova transição.
         let nt = parte_iv(inicio.clone());
         println!(
            "data: {}\nseleção: {:#?}\n",
            inicio.to_readable_string(),
            nt.file_name().unwrap()
         );
         // avançando dia ...
         inicio = inicio.next_date();
      }
      // conseguir atingir o que queria?
      assert!(false);
   }

   #[test]
   fn teste_manual_da_funcao_at() {
      alterna_transicao();
      assert!(true)
   }

   #[test]
   fn testa_duracao_atual_transicao() {
      let t = duracao_atual_transicao(); 
      assert!( t > Duration::from_secs(60));
      println!("valor={}", tempo(t.as_secs(), true));
   }
}
