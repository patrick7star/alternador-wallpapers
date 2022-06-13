
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
use utilitarios::{aleatorio, legivel};

// bibliotecas do Rust:
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::process::Command;

// próprios módulos:
mod banco_de_dados;
mod transicao;
mod atualizacoes;
mod comparacao;

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
   env!("HOME"),
   "/Documents/códigos_rust",
   "/personalização/data",
   "/ultima_escolha.txt"
);
/* caminho para novo arquivo que armazenará
 * tais registro de data. */
const CAMINHO_ARQUIVO:&str = concat!(
   env!("HOME"),
   "/Documents/códigos_rust",
   "/personalização/data",
   "/data_de_registro.dat" 
);

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
   .spawn()
   .unwrap();
}

fn main() {
  // marcando tempo inicial de contagem ...
   let mut cronometro = Instant::now();
   /* selecionando um 'tempo final' para que ao 
    * passar tal, aciona uma nova transição-de-
    * walpapers. Tal tempo 'tf' estará entre 
    * um MÁXIMO E MÍNIMO. */
   let tf:u16 = aleatorio::sortear::u16(MINIMO..=MAXIMO);
   let mut tempo_final = Duration::from_secs(tf as u64);
   let mut execucao_inicial:bool = false;
   /* toda vez que for acionada no começo de
    * do sistema/ou login uma nova 'transição
    * de wallpapers' será escolhidas baseado
    * na data de acionamento.
    * Agora, o programa também escolha uma nova 
    * 'transição', sem precisar nova inicialização
    * do sistema/ou login;... em média, de 5 à 8 
    * horas, decorrida da última mudança.  */
   loop {
      /* se tiver "atigindo" tal tempo, então
       * trocar a transição-de-wallpaper atual. */
      if cronometro.elapsed() > tempo_final && execucao_inicial {
         alterna_transicao();
         // zerá contador... para nova contagem.
         cronometro = Instant::now();
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
         let tempo_restante = tempo_final - cronometro.elapsed();
         // traduzindo segundos para algo legível.
         let str_tr = legivel::tempo(tempo_restante.as_secs(), true);
         println!("contagem regressiva para próxima transição {}", str_tr);
      }

      /* possívelmente realizando uma atualização
       * dos arquivlos de transições XMLs no
       * diretório onde ficam. */
      atualiza_xmls();

      // intercalar os loops a cada 30 segundos.
      sleep(Duration::from_secs(30_u64));
   }
}


