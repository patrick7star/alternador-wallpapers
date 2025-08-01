/*! 
    Alterna wallpapers, ou "exibição de wallpapers" durante tempos e tempos,
 contando com meses com comemorações específicas como o natal e halloween. 
 Ele aceita uma expansão dinâmica dos wallapapers que serão alternados, 
 apenas coloque mais nos diretórios específicos que ele lê. Será computado 
 em toda inicialização do computador, ou novo login.
*/

// Bibliotecas externas:
extern crate utilitarios;
// Módulos do próprio projeto:
mod banco_de_dados;
mod transicao;
mod atualizacoes;
mod comparacao;
mod compilacao;
mod notificacoes;
mod configuracao;
mod constantes;
// #[allow(unused)]
// mod transicoes;
mod comunicacao;

// Biblioteca externas:
#[allow(warnings)]
use utilitarios::aleatorio::sortear;
use utilitarios::legivel::{tempo as tempo_legivel};
use utilitarios::terminal::{terminal_largura, Largura};
// Bibliotecas do Rust:
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::env::var;
use std::path::{Path};
use std::process::{Command};
// Submódulos deste projeto:
use transicao::{
   alterna_transicao,
   duracao_atual_transicao
};
use atualizacoes::atualiza_xmls;
use constantes::*;


fn pausa_aleatoria() {
   /* Pausa de alguns minutos para se curtir a transição anterior. 
    * Claro, eles estão quantificados em segundos. */
   let minutos = {
      if cfg!(debug_assertions)
         { sortear::u64(3..=32) }
      else
         { sortear::u64(10*60..=15*60) }
   };
   let limite = Duration::from_secs(minutos);
   let timer = Instant::now();
   let taxa = || { 
      if cfg!(debug_assertions)
         { limite / 6 }
      else
         { Duration::from_secs(sortear::u64(9..=43)) }
    };

   while timer.elapsed() < limite {
      let restante = limite - timer.elapsed();

      println!(
         "Tempo de espera para iniciar de fato ...{:>10}", 
         tempo_legivel(restante.as_secs(), true)
      );
      sleep(taxa());
   }
}

fn desenha_cabecalho(msg: &str) {
   let Largura(a) = terminal_largura().unwrap();
   let b = msg.len();
   let n = ((a as usize - b) / 2) as usize;
   let bar = &"~".repeat(n);
   
   println!("{} {msg:} {}", bar, bar);
}

fn o_programa_de_transicao_existe(exe: &'static str) -> bool {
/* O programa pressume que tal programa tem uma opção 'version', assim como
 * a esmagadora maioria dos programas Linux tem. */
   let mut comando = Command::new(exe);

   comando.arg("--version");

   if let Ok(result) = comando.output() 
      { return result.status.success(); }
   // Se não retornado um valor válido, significa erro, logo confirma tese.
   false
}

/** Verificação e configuração de variáveis, diretórios, e etc. */
fn checagem_e_configuracao_do_ambiente() {
   let inputs = ["RUST_CODES", "XDG_CURRENT_DESKTOP"];

   desenha_cabecalho("Checagem de variáveis de ambientes estão okay");

   for variavel in inputs {
      print!("Variável '{variavel:}' presente ... ");

      if var(variavel).is_ok()
         { println!("\u{2714}"); }
      else
         { println!("\u{1f5d9}"); }
   }

   const GSETTINGS: &'static str = "/usr/bin/gsettings";
   let mut inputs = vec![
       PYTHON, ARQUIVO_CONF, ULTIMA_NOTIFICACAO, 
       CAMINHO_ARQUIVO, BD1
   ];
   desenha_cabecalho("Verificação de todos caminhos do código");

   for caminho_str in inputs.drain(..) {
      let caminho = Path::new(caminho_str);

      print!("{caminho_str:} ... ");
      if caminho.exists()
         { println!("\u{2714}"); }
      else
         { println!("\u{1f5d9}"); }
   }

   print!("{} ...", GSETTINGS); 

   if o_programa_de_transicao_existe(GSETTINGS) 
      { println!("\u{2714}"); }
   else
      { println!("\u{1f5d9}"); }
   // Separando 'output' dos demais ...
   print!("\n\n");
}

fn execucao_continua_da_transicao() {
   let n = sortear::u16(MINIMO..=MAXIMO) as u64;
   let mut tempo_final = Duration::from_secs(n);
   let mut execucao_inicial = false;
   let mut cronometro = Instant::now();

   loop {
      let aciona_uma_nova_transicao = {
      /* Se tiver "atigindo" tal tempo, então trocar a transição de 
       * wallpaper atual. */
         cronometro.elapsed() > tempo_final
                     &&
         execucao_inicial
      };

      if aciona_uma_nova_transicao {
         alterna_transicao();
         // Zerá contador... para nova contagem.
         cronometro = Instant::now();
         /* Pegando duração do tempo total de apresentação da nova 
          * transição de slides, somado à 1min para acabar toda 
          * apresentação. */
         tempo_final = duracao_atual_transicao() + Duration::from_secs(60);
         // Mostra a notificação da atual ação.
         notificacoes::popup_notificacao_de_transicao();

      } else if !execucao_inicial {
         // faz uma execução inicial
         alterna_transicao();
         // mostra a notificação da atual ação.
         notificacoes::popup_notificacao_de_transicao();
         // executada uma vez ...
         execucao_inicial = true;

      } else {
         // obtendo tempo para próxima transição.
         let tempo_restante = tempo_final - cronometro.elapsed();
         // traduzindo segundos para algo legível.
         println!(
            "contagem regressiva para próxima transição {}",
            tempo_legivel(tempo_restante.as_secs(), true)
         );
      }

      /* Possívelmente realizando uma atualização dos arquivlos de 
       * transições XMLs no diretório onde ficam. */
      if false { atualiza_xmls(); } // desabilitado por enquanto

      // intercalar os loops à cada, aproximadamente, 1 min.
      sleep(Duration::from_secs(sortear::u64(30..=60)));
   }
}

fn main() {
   checagem_e_configuracao_do_ambiente();

   /* se for o artefato de depuração, então já colocar em caminho uma 
    * possível compilação da versão otimizada. */
    if compilacao::executa_compilacao()
      { println!("Uma compilação foi realizada!"); }
   else
      { println!("Nenhuma compilação foi inicializada!"); }

   /* Antes de começar uma nova transição, ou a comunicação exterior; o 
    * programa simplesmente dá uma breve pausa, prá se ver, e também apreciar
    * o wallpaper escolhido antes do último desligamento ou login realizado
    * antes desta nova execução. */
   pausa_aleatoria();

   /* Diz para processos externos, deste programa, que não é necessário
    * lançar uma nova instância, porque uma(esta) já está rodando. Ambas
    * são threads. */
   let cliente = comunicacao::receptor();
   let _servidor = comunicacao::transmissor(cliente);

   execucao_continua_da_transicao();
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   
   #[test]
   fn visualizacao_do_output_da_checagem_de_variaveis() {
      checagem_e_configuracao_do_ambiente();
   }

   #[test]
   #[cfg(target_os="linux")]
   fn existencia_do_gsettings() {
      let programa = "/usr/bin/gsettings";

      print!("O programa Gsettings existe? ");
      assert!(o_programa_de_transicao_existe(programa));
      println!("Sim.");
   }
}
