
/*! 
 Alterna wallpapers, ou "exibição de wallpapers" 
durante tempos e tempos, contando com meses com 
comemorações específicas como o natal e halloween. 
Ele aceita uma expansão dinâmica dos wallapapers 
que serão alternados, apenas coloque mais nos 
diretórios específicos que ele lê. Será computado
em toda inicialização do computador, ou novo login.
*/

// importando minha biblioteca ...
extern crate utilitarios;
use utilitarios::aleatorio::sortear;
use utilitarios::legivel::tempo as tempo_l;

// bibliotecas do Rust:
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

// próprios módulos:
mod banco_de_dados;
mod transicao;
mod atualizacoes;
mod comparacao;
mod temporizador;
mod compilacao;

use temporizador::*;

use transicao::{
   alterna_transicao,
   duracao_atual_transicao
};
use atualizacoes::atualiza_xmls;

/* o máximo e mínimo de tempo que deve
 * ser selecionada uma nova transição-
 * de-imagens é entre 5h e 8h. */
const MINIMO:u16 = 1_600;
const MAXIMO:u16 = 3_600;
/* caminho do diretório que será trabalhado.
 * diretório onde será varrido por 
 * slides-de-transição. */
const RAIZ:&str = concat!(env!("HOME"), "/Pictures");
// registros de mudanças feitas.
const BD1:&str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/ultima_escolha.txt"
);
/* caminho para novo arquivo que armazenará
 * tais registro de data. */
const CAMINHO_ARQUIVO:&str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/data_de_registro.dat" 
);
// atalho para o binário do Python.
const PYTHON:&'static str = "/usr/bin/python3";

// extensão para o objeto String.
trait Extensao {
   fn titulo(&self) -> String;
}
 
impl Extensao for String {
   /* faz uma string com um título, ou seja,
    * faz cada palavra "capitalizada". */
   fn titulo(&self) -> String {
      let mut nova_str = String::new();
      for s in self.split(' ') {
         let primeiro_char = s.get(0..1).unwrap();
         nova_str += primeiro_char.to_uppercase().as_str();
         nova_str += s.get(1..).unwrap();
         nova_str.push(' ');
      }
      return nova_str;
   }
}

fn atual_transicao() -> String {
   match banco_de_dados::le_escolha() {
      Ok(caminho_contido) => {
         let string = {
            caminho_contido
            .as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap_or("ALGO FOI ERRADO".to_string())
         };
         /* retirando o traço por espaço
          * e colocando tudo maiúscula. */
         string
         .replace("_", " ")
         .replace(".xml", "")
         .titulo()
         .trim_end()
         .to_string()
      } Err(_) => 
         { panic!("erro ao extraír caminho!"); }
   }
}

/* faz uma notificação da atual transição
 * aplicada ao sistema. */
fn popup_notificacao() {
   /* obtêm o nome da atual transição, e trabalha
    * um pouquinho nela. */
   let nome_transicao = atual_transicao();
   // notificação sobre transição.
   let mensagem = format!(
      "a nova transição de imagem \"{}\" foi colocada",
      nome_transicao
   );
   let argumentos:[&str; 4] = [
      "--expire-time=25000",
      "--icon=object-rotate-left",
      "--app-name=AlternaWallpaper",
      mensagem.as_str()
   ];

   // executando comando ...
   Command::new("notify-send")
   .args(argumentos.into_iter())
   .spawn().unwrap()
   .wait().unwrap();
   println!("notificação foi \"plotada\" com sucesso.");
}

fn pausa_aleatoria() {
   /* pausa de alguns minutos para se curtir 
    * a transição anterior. Claro, eles estão
    * quantificados em segundos. */
   let minutos = sortear::u64(10*60..=15*60);

   // informando tempo de espera(te).
   let timer = Temporizador::novo(Duration::from_secs(minutos));
   while !timer.esgotado() {
      println!(
         "tempo de espera para iniciar de fato ...{:>10}", 
         tempo_l(timer.contagem().as_secs(), true)
      );
      let segundos = sortear::u64(9..=43);
      let t = Duration::from_secs(segundos);
      sleep(t.clone());
   }
}

fn main() {
   /* se for o artefato de depuração, então 
    * já colocar em caminho uma possível 
    * compilação da versão otimizada. */
    if compilacao::executa_compilacao()
      { println!("uma compilação foi realizada!"); }
   else
      { println!("nenhuma compilação foi inicializada!"); }

   // marcando tempo inicial de contagem ...
   let mut cronometro = Cronometro::novo();
   /* selecionando um 'tempo final' para que ao 
    * passar tal, aciona uma nova transição-de-
    * walpapers. Tal tempo 'tf' estará entre 
    * um MÁXIMO E MÍNIMO. */
   let tf:u16 = sortear::u16(MINIMO..=MAXIMO);
   let mut tempo_final = Duration::from_secs(tf as u64);
   let mut execucao_inicial = false;

   /* toda vez que for acionada no começo de
    * do sistema/ou login uma nova 'transição
    * de wallpapers' será escolhidas baseado
    * na data de acionamento.
    * Agora, o programa também escolha uma nova 
    * 'transição', sem precisar nova inicialização
    * do sistema/ou login;... em média, de 5 à 8 
    * horas, decorrida da última mudança.  */
   pausa_aleatoria();

   loop {
      /* se tiver "atigindo" tal tempo, então
       * trocar a transição-de-wallpaper atual. */
      let aciona_uma_nova_transicao = {
         //cronometro.elapsed() > tempo_final 
         cronometro > tempo_final
                     &&
         execucao_inicial
      };

      if aciona_uma_nova_transicao {
         alterna_transicao();
         // zerá contador... para nova contagem.
         //cronometro = Instant::now();
         cronometro.reseta();
         /* pegando duração do tempo total de apresentação
          * da nova transição de slides, somado à 1min para 
          * acabar toda apresentação. */
         tempo_final = {
            duracao_atual_transicao() 
                     + 
            Duration::from_secs(60)
         };
         // mostra a notificação da atual ação.
         popup_notificacao();
      } else if !execucao_inicial {
         // faz uma execução inicial
         alterna_transicao();
         // mostra a notificação da atual ação.
         popup_notificacao();
         // executada uma vez ...
         execucao_inicial = true;
      } else {
         // obtendo tempo para próxima transição.
         //let decorrido = cronometro.elapsed();
         let decorrido = cronometro.marca();
         let tempo_restante = tempo_final - decorrido;
         // traduzindo segundos para algo legível.
         println!(
            "contagem regressiva para próxima transição {}",
            tempo_l(tempo_restante.as_secs(), true)
         );
      }

      /* possívelmente realizando uma atualização
       * dos arquivlos de transições XMLs no
       * diretório onde ficam. */
      atualiza_xmls();

      // intercalar os loops à cada, aproximadamente, 1 min.
      sleep(Duration::from_secs(sortear::u64(30..=60)));
   }
}

