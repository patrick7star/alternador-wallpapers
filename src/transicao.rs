extern crate date_time;
// contiação do módulo ...
mod embaralhamento;
mod datas_especiais;
mod resolve_repeticoes;

// Biblioteca padrão do Rust:
use std::str::FromStr;
use std::time::Duration;
use std::env;
use std::path::{PathBuf};
use std::ffi::{OsString, OsStr};
use std::process::{Output, Command};
// Própria lib.
use super:: banco_de_dados::{grava_escolha, le_escolha};
use crate::constantes::{RAIZ, PYTHON};
// Bibliotecas externas:
pub use date_time::date_tuple::DateTuple;

/* também re-exporta função para não ter
 * que importar aqui também. */
use embaralhamento::{sortear, embaralha};
use super::configuracao::{raiz_wallpapers, wallpapers_externos};

type Periodo = (DateTuple, DateTuple);
type Conclusao = Option<f32>;


/* acha todos XML que contém uma transição programada de determinados 
 * wallpapers, geralmente localizados no mesmo diretório de tal. */
pub fn parte_i() -> Vec<PathBuf> {
   // todos arquivos e subdiretórios da RAIZ dada.
   let sua_localizacao = raiz_wallpapers().read_dir().unwrap();

   /* pegando já "wallpapers externos" à raiz, porém filtrando apenas
    * caminhos existentes. */
   let mut arquivos_xml = {
      wallpapers_externos().drain(..)
      .filter(|path| path.exists())
      .collect::<Vec<PathBuf>>()
   };

   // varrendo tal raíz, e colhendo arquivos XML's...
   for dir in sua_localizacao {
      let entrada = dir.unwrap().path();
      let diretorio = entrada.as_path();
      /* se for um diretório, entra nele e varre-o por um padrão, que 
       * é: verificar se há arquivos de imagens e um xml com o nome do 
       * diretório. O algoritmo, por enquanto, só entra um subdiretório
       * atrás de XML's, futuramente a busca será mais profunda. */
      if diretorio.is_dir() {
         // novo iterador de entradas do subdiretório.
         let novas_dir = diretorio.read_dir().unwrap();

         for subdir in novas_dir {
            let entrada = subdir.unwrap().path();
            let caminho_i = entrada.as_path();
            let extensao = {
               caminho_i.extension()
               .unwrap_or(OsStr::new("nada"))
               /*
               Some(string) => string,
               None => OsStr::new("nada"),*/
            };

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
   if todos.is_empty()
      { panic!("nenhum arquivo XML de transição foi encontrado!"); }
   // embalharando dados extraídos.
   embaralha(&mut todos);
   // qual indexar na array.
   let ultimo: u8 = (todos.len()-1) as u8;
   let numero_sorteado = sortear::u8(0..=ultimo);
   // retornando seleção.
   return todos[numero_sorteado as usize].clone();
}

/* computa o percentual quão decorreu 
 * até o momento no período do feriado.
 */
fn percentual(hoje: DateTuple, periodo: Periodo) -> Conclusao {
   // não se pode computar ainda ...
   // depois do período.
   if hoje.to_days() > periodo.1.to_days()
      { return None; }
   // antes do ínicio.
   else if hoje.to_days() < periodo.0.to_days()
      { return None; }

   let a = hoje.to_days() - periodo.0.to_days();
   let t = periodo.1.to_days() - periodo.0.to_days();
   Some((a as f32) / (t as f32))
}

/* retorna o booleano passado, dependendo
 * apenas de quão perto do feriado
 * está. Se está alguns dias, ou no 
 * dia, é certeza que aparecerá os wallpapers
 * especiais para o feriado;  se está 
 * entre 50 à 25 porcento longe do feriádo
 * a probabilidade de um wallpaper do feriado
 * aparecer é 70%; já se está menos da 
 * metade de dias do feriado, a probabilidade
 * cai para 50%, ou seja, pode, ou não
 * aparecer o wallpaper para ele. */
fn avalia_booleano(percentual: f32, valor: bool) -> bool {
   // baseado no período de conclusão.
   if percentual < 0.50 {
      if sortear::bool() { valor }
      else 
         { false }
   } else if percentual >= 0.50 && percentual < 0.75 {
      // probabilidade de 70% porcento.
      if sortear::u8(1..=10) <= 7 { valor }
      else
         { false }
   }
   else 
      // passou de 80% do periódo validação como certa.
      { valor }
}

/* 
 * Faz uma seleção levando transições de 
 * datas especiais em consideração na seleção.
 * Usa a função acima em consideração na 
 * seleção randômica.
*/
#[allow(dead_code)]
fn parte_iii(hoje:DateTuple) -> PathBuf {
   // obtem uma transição antes.
   let transicao = parte_ii();
   /* trabalha, inicialmente, em dois casos
    * especiais: Halloween e Natal.  */
   let mes = hoje.get_month();
   let dia = hoje.get_date();
   let ano = hoje.get_year();

   // Halloween ou próximo dele.
   let e_periodo_de_halloween:bool = {
      let periodo = (
         DateTuple::new(ano, 9, 28).unwrap(),
         DateTuple::new(ano, 10, 31).unwrap()
      );
      let valor_logico = {
         (dia >= 28 && mes == 9) ||
         (dia >= 1 && dia <= 31 && mes == 10) 
      };
      match percentual(hoje.clone(), periodo) {
         Some(p) => {
            avalia_booleano(p, valor_logico)
         } None => 
            { valor_logico }
      }
   };
   // Natal ou próximo dele.
   let e_periodo_de_natal: bool = {
      let periodo = (
         DateTuple::new(ano, mes, 1).unwrap(),
         DateTuple::new(ano, mes, 25).unwrap()
      );
      
      // baseado no período de conclusão.
      match percentual(hoje.clone(), periodo) {
         Some(p) => {
            avalia_booleano(p, mes == 12 && dia >= 1 && dia <= 25 )
         } None => 
            { mes == 12 && dia >= 1 && dia <= 25 }
      }
   };
   // Aniversário de Brasília.
   let e_aniversario_de_brasilia:bool = {
      let periodo = (
         DateTuple::new(ano, mes, 10).unwrap(),
         DateTuple::new(ano, mes, 21).unwrap()
      );
      let valor_logico = {
         mes == 4 && 
         dia >= 10 && 
         dia <= 21
      };
      match percentual(hoje.clone(), periodo) {
         Some(p) => {
            avalia_booleano(p, valor_logico)
         } None => 
            { valor_logico }
      }
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
   } else { 
      // todos demais caem neste caso.
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
#[allow(dead_code)]
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

/* Ajusta encaixe da melhor, baseado na transição que foi escolhida. */
fn parte_v(caminho:PathBuf) -> PathBuf {
   let caminho_i = caminho.as_path();
   let nome = caminho_i.file_name().unwrap();

   // ambiente que está inserido.
   let ambiente = match env::var("XDG_CURRENT_DESKTOP") {
      Ok(a) => {
         if a == "ubuntu:GNOME"
            { "org.gnome.desktop.background" }
         else if a == "MATE"
            { "org.mate.background" }
         else
            { panic!("não implementado para tal ambiente!"); }
      } Err(_) => 
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

   // rodando comando que troca opção da imagem ...
   Command::new("gsettings")
   .arg("set")
   .arg(ambiente)
   .arg("picture-options")
   .arg(opcao)
   .spawn()
   .unwrap()
   .wait()
   .unwrap();

   // retornando caminho obtido ...
   return caminho;
} 

/* Trecho cuida essencialmente da mudança da atual transição para uma nova
 * que é passada como argumento. */
fn executa_transicao_de_wallpaper(input: PathBuf, input_a: PathBuf) {
	let caminho = input.as_path();
	// Tupla de argumentos levando em conta o ambiente gráficos.
   let (chave, atributo) =  match env::var("XDG_CURRENT_DESKTOP")
	{
		Ok(ambiente_grafico) => {
			if ambiente_grafico == "ubuntu:GNOME"
				{ ("org.gnome.desktop.background", "picture-uri") }
			else if ambiente_grafico == "MATE"
				{ ("org.mate.background", "picture-filename") }
			else { 
            panic!(
               "não implementado para tal ambiente '{}'!", 
               ambiente_grafico
            ); 
         }
		} Err(erro_msg) => { panic!("Possível error: {erro_msg:}"); }
	};

	/* Constituindo o comando que roda, então executando ele ...*/
   Command::new("gsettings")
	.args(["set", chave, atributo, caminho.to_str().unwrap()].into_iter())
   .spawn().unwrap().wait().unwrap();

	/* Mensagem explicando o que foi realizado acima. */
	let nova_selecao = input.file_name().unwrap();
	let ultima_selecao = input_a.file_name().unwrap();
   println!(
      "\nAlternância transição-de-wallpapers automaticamente acionada.
      \r\tSeleção anterior: {:?}
      \r\tArquivo selecionado: {:?}\n",
		ultima_selecao, nova_selecao
   );
}

/** Executa o comando de troca de wallpapers acionando uma 
  * transição-de-wallpapers já pré-configurada. */
pub fn alterna_transicao() {
   let data_de_hoje = DateTuple::today();
   let nova_transicao = parte_v(resolve_repeticoes::parteIV(data_de_hoje));
	
	if let Ok(ultima_selecao) = le_escolha() {
		executa_transicao_de_wallpaper
			(nova_transicao, ultima_selecao);
	}
}

/** Pega tempo da atual transição em segundos e retorna um 'Duration'. */
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
      "/alternador-wallpapers/",
      "extern_lib/slide_background/xml_info.py"
   );
   // array com bytes do resultado!
   let resultado: Output = {
      Command::new(PYTHON)
      .arg(caminho_script)
      .arg(escolha)
      .output()
      .unwrap()
   };
   let mut decimal = String::new();

   // Removendo quebra-de-linha ...
   // resultado.stdout.pop().unwrap();

   // Formando string com bytes representando número decimal.
   for byte in resultado.stdout.iter() 
      { decimal.push(*byte as char); }
   
   // Convertendo para um decimal ponto-flutuante ...
   match f32::from_str(decimal.as_str()) {
      // Criando 'Duration' com valor recuperado ...
      Ok(segundos) => Duration::from_secs_f32(segundos),
      /* Em caso de erro, sorteio um perído entre uma hora à quatro horas. */
      Err(_) => {
         // limites inferior e superior(1h à 4h).
         let (li, ls): (u64, u64) = (3600, 4*3600);
         let tempo = sortear::u64(li..=ls);
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
   use utilitarios::legivel::tempo_legivel;
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
      println!("valor={}", tempo_legivel(t.as_secs(), true));
   }
}
