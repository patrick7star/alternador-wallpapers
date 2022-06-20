
/*!
 De período em período, vai obter todos
os XML's e, possíveis novos diretórios e 
atualixar XML's, para que se novas imagens
forem adicionadas ou deletadas, os arquivos
de transição tem isso em mente. 

 O tempo de tais atualizações vai variar em
dadas condições, a cada três dias, eles
vasculham e atualizam, com o passar do tempo
isso será numa frequência semanal e talvez
até mensal de for dado que não há muita
alteração do 'status' atual.

 O programa que fará tais atualizações é feito
em Python, e não Rust, porque para fazer isso
em tal linguagem teria que ser feito todo a
base que já faz isso e, foi feita inicialmente
em Python. Estará junto no diretório das 
'bibliotecas externas'.
*/

// importando do caixote principal.
use super::{CAMINHO_ARQUIVO, comparacao};

// bibliotecas externas.
extern crate xshell;
extern crate date_time;
extern crate utilitarios;
use xshell::Cmd;
use date_time::date_tuple::DateTuple;
use utilitarios::legivel;

// biblioteca do Rust:
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::env;
use std::ffi::OsStr;
use std::str::FromStr;
use std::path::PathBuf;


pub trait Serializacao {
   /* pega o DateTuple e converte numa 
    * array de bytes, onde o primeiro e 
    * segundo bytes representam, cada, 
    * dia e mês, já os dois últimos representam
    * o ano. */
   fn serializa(self) -> [u8; 2+1+1];
   /* transforma o "linguição de bytes" em valores.
    * o primeiro valor é o dia, o segundo é mês
    * e o último é o ano. */
   fn deserializa(bytes:[u8; 4]) -> DateTuple;
}

impl Serializacao for DateTuple {
   fn serializa(self) -> [u8; 4] {
      // array vázia inicialmente.
      let mut linguicao:[u8; 4] = [u8::MAX;4];
      // primeiro armazena o dia.
      linguicao[0] = self.get_date();
      // agora obtem o outro byte representando o mês.
      linguicao[1] = self.get_month();
      // os dois bytes(valor 16-bits) representando o ano.
      let bytes_do_ano = self.get_year().to_be_bytes(); 
      linguicao[2] = bytes_do_ano[0];
      linguicao[3] = bytes_do_ano[1];
      return linguicao;
   }

   fn deserializa(bytes:[u8; 4]) -> DateTuple {
      let dia:u8 = bytes[0];
      let mes:u8 = bytes[1];
      let buffer:[u8;2] = [bytes[2], bytes[3]];
      let ano:u16 = u16::from_be_bytes(buffer);
      // criando o tipo de dado.
      return DateTuple::new(ano, mes, dia).unwrap();
   }
}

fn registra_no_bd(data:DateTuple) {
   let mut banco_de_dados:File = {
      OpenOptions::new()
      .create(true)
      .write(true)
      .open(CAMINHO_ARQUIVO)
      .unwrap()
   };
   // escrevendo dados em disco ...
   banco_de_dados
   .write(&data.serializa()[..])
   .unwrap();
}

fn recupera_do_bd() -> DateTuple {
   let mut banco_de_dados:File = {
      match OpenOptions::new()
      .read(true)
      .open(CAMINHO_ARQUIVO) {
         Ok(file) => file,
         Err(_) => {
            let hoje = {
               DateTuple::today()
               .next_date()
               .next_date()
               .next_date()
               .next_date()
            };
            registra_no_bd(hoje);
            panic!("não havia arquivos com datas registradas!");
         },
      }
   };
   // escrevendo dados em disco ...
   let mut bytes:[u8; 4] = [0, 0, 0, 0];
   banco_de_dados
   .read(&mut bytes)
   .unwrap();
   return DateTuple::deserializa(bytes);
}

fn e_hora_de_atualizar(hoje:DateTuple, 
ultima_atualizacao:DateTuple, numero_de_dias:u8) -> bool {
   /* se for uma data maluca, que passa 
    * a data atual de mode extremo, então
    * autorizar a atualização agora. */
   let mes = hoje.get_month();
   let dia = hoje.get_date();
   let perto_do_natal:bool =  {
      dia >= 1 && dia <= 18 && 
      mes == 12
   };
   let perto_do_halloween:bool = {
      (mes == 09 && dia >= 25) ||
      (mes == 10 && dia >= 1 && dia <= 25)
   };
   let variavel:&str = "ATUALIZADO_HOJE";

   /* se do banco, a última atualização for uma 
    * data futura, então, provavelmente o arquivo
    * "está quebrado", e uma atualização agora
    * concerta isso. */
   if ultima_atualizacao > hoje 
      { return true; }
   /* na proximidade de algumas datas comemorativas
    * trabalha diariamente, até menos do que isso.
    */
   else if perto_do_halloween || perto_do_natal {
      // o que a variável de ambiente diz?
      let ja_foi_atualizado:bool = {
         /* try fetch 'ATUALIZADO_HOJE' value
          * if find nothing, then return false
          * as value else try parse string retrived. */
         match env::var(variavel) {
            Ok(valor) =>  
               { bool::from_str(valor.as_str()).unwrap() }, 
            Err(_) => false,
         }
      };
      /* se não foi dá sinal, caso contrário
       * mande parar. */
      if !ja_foi_atualizado {
         // cria variável.
         env::set_var(OsStr::new(variavel), OsStr::new("true"));
         // dá sinal para atualização.
         return true; 
      } else { return false; }
   }
   /* se a contabilidade falhar, então é hora de
    * atualizar. */
   //else if comparacao::contabilidade_esta_ok()
   //   { return true; }
   /* no caso clássico de a "última atualização"
    * ter sido feita, ou hoje, ou a "tempos"
    * atrás, vamos analisar com mais calma o 
    * período que se extende. */
   else {
      // alias para simplificar legibilidade.
      let anos_de_diferenca:u16 = {
         let ai = ultima_atualizacao.get_year() as i32;
         let af = hoje.get_year() as i32;
         // retornando a diferença.
         (ai-af).abs() as u16
      };
      let dm:u8 = {
         let mf = hoje.get_month();
         let mi = ultima_atualizacao.get_month();
         if mf == 1 && mi == 12 { 1 }
         else if mi > mf 
            { mi - mf }
         else { mf - mi }
      };

      // se for igual, não fazer nada.
      if ultima_atualizacao == hoje
         { return false; }
      // se houver decorrido mais de cinco meses ou anos.
      else if dm >= 5 || anos_de_diferenca >= 1
         { return true; }
      // onde queríamos chegar finalmente!
      else {
         let df = hoje.to_days();
         let di = ultima_atualizacao.to_days();
         if (df - di) as u8 >= numero_de_dias 
            { return true; }
         else 
            { return false; }
      }
   }
}

/** verifica se decorreu um período pré-estabelecido,
 tirando datas especiais, onde tal será bem mais 
 curto, chegando até ser diário ou menos às vezes;
 ele executará um script em Python que faz as devidas
 atualizações das transições de imagens em XML, que 
 estão no diretório Pictures na $HOME. 
*/
pub fn atualiza_xmls() {
   // lê último data de atualização do BD.
   let ultima = recupera_do_bd();
   // obtem a data de hoje.
   let hoje = DateTuple::today();
   // o rítmo de dias para cada atualização(10 dias).
   let ritmo = 10;
   // criando comando para executar o script:
   // formando caminho ...
   let mut caminho:PathBuf = PathBuf::new();
   caminho.push(env!("RUST_CODES"));
   caminho.push("personalização");
   caminho.push("extern_lib");
   caminho.push("slide_background");
   caminho.push("atualiza_configuracao_xml.py");
   // formando o comando ...
   let comando:xshell::Cmd = {
      Cmd::new("/usr/bin/python3")
      .arg(OsStr::new("-O"))
      .arg(caminho.as_os_str())
      .echo_cmd(false)
   };

   /* roda o sinal e informa o resultado, se
    * a função dá o sinal; ou número de wallpapers
    * contado "diretamente" no diretório, e no 
    * arquivo XML também têm de ser iguais. */
   if e_hora_de_atualizar(hoje, ultima, ritmo) || 
   !comparacao::contabilidade_esta_ok() { 
      // informação sobre contagem de wallpapers.
      println!("{}\natualizando ...", comparacao::razao_info());
      // roda comando ...
      match comando.run() {
         Ok(_) => { 
            // marca como feito.
            println!("feita COM SUCESSO."); 
            // registra data de útlima atualização.
            registra_no_bd(hoje.clone());
         },
         Err(_) =>
            { println!("um ERROR ocorreu!"); }
      }
   } else { 
      /* obtendo dias que falta para nova atualização 
       * dos XML's, alias para melhorar legibilidade. */
      let dh = hoje.to_days();
      let ua = ultima.to_days();
      /* o cálculo é o rítmo menos os dias
       * decorridos desde á última atualização
       * relaizada. */
      let dias_restantes = (ritmo as u32) - (dh - ua);
      let dias_restantes:u64 = (dias_restantes as u64) * 3600 * 24;
      // mensagem de negação, e tempo restantes.
      println!(
         "atualização NÃO AUTORIZADA!
         \rtempo restante é {}\n",
         legivel::tempo(dias_restantes, false)
      ); 
   }
}


#[cfg(test)]
mod tests {
   use super::{
      DateTuple, Serializacao, env,
      registra_no_bd, recupera_do_bd,
      CAMINHO_ARQUIVO, e_hora_de_atualizar,
      atualiza_xmls, xshell, PathBuf,
   };
   use std::path::Path;
   use std::fs::{copy, remove_file, rename};
   use std::thread;
   use std::time::{Duration};

   /* Função pega o arquivo de registro, e cria um 
    * backup para ele, basicamente ao lado. Quando
    * for chamado novamente, e ver tal backup, então
    * remove a entrada antiga, e renomea o backup 
    * para seu nome original. */
   fn salva_recupera() {
      // nome do backup.
      let backup = format!("{}.bak",CAMINHO_ARQUIVO);
      // proposições sobre a existência dos arquivos.
      let backup_existe:bool = {
         let slice_str = backup.as_str();
         Path::new(slice_str)
         .exists()
      };
      let arquivo_original_existe:bool = {
         let slice_str = CAMINHO_ARQUIVO;
         Path::new(slice_str)
         .exists()
      };

      if backup_existe && arquivo_original_existe { 
         let nome_antigo = Path::new(backup.as_str());
         let novo_nome = Path::new(CAMINHO_ARQUIVO);
         /* deleta novo backup que foi renomeado com 
          * este nome para dá espaço ao arquivo 
          * antigo. */
         remove_file(novo_nome).unwrap();
         // renomea backup feito.
         rename(nome_antigo, novo_nome).unwrap() 
      } 
      else if !backup_existe && arquivo_original_existe {
         /* renomea o antigo com extensão de 
          * backup. */
         let novo = Path::new(backup.as_str());
         let antigo = Path::new(CAMINHO_ARQUIVO);
         rename(antigo, novo).unwrap();
         /* copia o com extensão backup como 
          * o novo arquivo com extensão final
          * dot "dat". */
         let destino = Path::new(CAMINHO_ARQUIVO);
         let origem = Path::new(backup.as_str());
         copy(origem, destino).unwrap();
      } 
      else if backup_existe && !arquivo_original_existe {
         /* este caso o processo de recuperar o arquivo 
          * original falhou. Então realizar a renomeação
          * e começar tudo novamente. */
         let antigo = backup.as_str();
         let novo = CAMINHO_ARQUIVO;
         rename(antigo, novo).unwrap();
         // jogando para o segundo caso.
         salva_recupera();
      }
      else { 
         /* neste último, para o programa! É preciso a 
          * existência do primeiro no mínimo. */
         panic!(
            "não é possível continuar sem o arquivo \"{}\" e \"{}\"!",
            CAMINHO_ARQUIVO, backup
         ); 
      }
   }

   #[test]
   fn serializacao_teste() {
      let d1 = DateTuple::new(1985, 5, 23);
      let d2 = DateTuple::new(1992, 3, 11);
      let d3 = DateTuple::new(1967, 9, 07);
      println!("{:?}", d1.unwrap().serializa());
      println!("{:?}", d2.unwrap().serializa());
      println!("{:?}", d3.unwrap().serializa());
      assert!(true);
   }

   #[test]
   fn deserializa_teste() {
      /* array representando os dados serializados
       * do teste acima. */
      let s = [23, 5, 7, 193];
      let d = DateTuple::deserializa(s);
      println!("{:#?}", d);
      assert_eq!(d.get_year(), 1985);
      assert_eq!(d.get_month(), 5);
      assert_eq!(d.get_date(), 23);

      let s = [11, 3, 7, 200];
      let d = DateTuple::deserializa(s);
      println!("{:#?}", d);
      assert_eq!(d.get_year(), 1992);
      assert_eq!(d.get_month(), 3);
      assert_eq!(d.get_date(), 11);

      let s = [7, 9, 7, 175];
      let d = DateTuple::deserializa(s);
      println!("{:#?}", d);
      assert_eq!(d.get_year(), 1967);
      assert_eq!(d.get_month(), 9);
      assert_eq!(d.get_date(), 7);
      // amostra aleatória.
      let s = [2, 8, 27, 193];
      let d = DateTuple::deserializa(s);
      println!("{:#?}", d);
      assert_eq!(d.get_month(), 8);
      assert_eq!(d.get_date(), 2);
   }

   #[test]
   fn testa_registro_no_bd() {
      // faz backup do arquivo que será modificado.
      salva_recupera();
      // data aleatória para teste de quase um ano.
      let da = DateTuple::new(2001, 5, 8).unwrap();
      println!("seguintes bytes:{:?}",da.serializa());
      registra_no_bd(da);
      assert!(true);
      // restura backup.
      salva_recupera();
   }

   #[test]
   #[should_panic]
   fn programa_para_sem_arquivo() {
      salva_recupera();
      // primeiro remove arquivo se há.
      match remove_file(CAMINHO_ARQUIVO) {
         Ok(_) => 
            { println!("arquivo foi excluído com sucesso."); }
         Err(_) => 
            { println!("nada foi feito!"); }
      }
      /* tenta recuperar dados de um arquivo
       * já excluído. */
      recupera_do_bd();
   }

   #[test]
   fn testa_recupera_do_bd() {
      salva_recupera();
      let da = DateTuple::new(2999, 12, 31).unwrap();
      registra_no_bd(da);
      // intervalo proposital.
      thread::sleep(Duration::from_secs(5));
      let dado_recuperado = recupera_do_bd();
      assert_eq!(
         dado_recuperado.serializa(),
         [31, 12, 11, 183]
      );
      /* vários dados, sobrepondo uns aos 
       * outros para que verifica erro, pois
       * ficou algum resto na última atualização.
       */
      let da = DateTuple::new(2001, 5, 8).unwrap();
      registra_no_bd(da);
      thread::sleep(Duration::from_secs_f32(0.5));
      let da = DateTuple::new(1920, 10, 8).unwrap();
      registra_no_bd(da);
      thread::sleep(Duration::from_secs_f32(3.5));
      let da = DateTuple::new(1956, 1, 25).unwrap();
      registra_no_bd(da);
      thread::sleep(Duration::from_secs_f32(1.5));
      let dado_recuperado = recupera_do_bd();
      assert_eq!(
         dado_recuperado.serializa(),
         [25, 1, 7, 164]
      );
      /* mesma coisa, porém com dados similares. 
       * E também gravações em tempos seguidos, para
       * tentar incitar ao ERRO. */
      let da = DateTuple::new(1979, 11, 04).unwrap();
      registra_no_bd(da);
      let da = DateTuple::new(1979, 11, 03).unwrap();
      registra_no_bd(da);
      let dado_recuperado = recupera_do_bd();
      assert_eq!(
         dado_recuperado.serializa(),
         [03, 11, 7, 187]
      );
      salva_recupera();
   }

   #[test]
   fn testa_e_hora_de_atualizar() {
      // testando uma data futura.
      let h = DateTuple::today();
      let ua = DateTuple::new(2051, 02, 27).unwrap();
      assert!(e_hora_de_atualizar(h, ua, 3));

      // testando um passado superior a um ano.
      let h = DateTuple::new(2022, 02, 28).unwrap();
      let ua = DateTuple::new(2021, 02, 27).unwrap();
      assert!(e_hora_de_atualizar(h, ua, 19));

      let ua = DateTuple::new(2001, 01, 07).unwrap();
      assert!(e_hora_de_atualizar(h, ua, 19));

      // testando um passado superior a cinco meses.
      let ua = DateTuple::new(2001, 01, 01).unwrap();
      let h = DateTuple::new(2001, 05, 30).unwrap();
      let diferenca:u8 = (h.to_days() - ua.to_days()) as u8;
      assert!(e_hora_de_atualizar(h, ua, diferenca));

      // passando o limite em alguns dias para receber um NÃO.
      let nh = DateTuple::new(2001, 05, 29).unwrap();
      let diferenca:u8 = (nh.to_days() - ua.to_days()) as u8;
      assert!(e_hora_de_atualizar(nh, ua, diferenca));
      
      // não dá permição para atualizar no mesmo dia.
      let h = DateTuple::new(2001, 05, 30).unwrap();
      let ua = h.clone();
      assert!(!e_hora_de_atualizar(h, ua, 7));

      /* dá licença de atualização a cada dez
       * dias, com exceção perto de feriados, onde
       * tal ritmo cai para diário. */
      let mut h = DateTuple::new(1911, 07, 5).unwrap();
      let mut ua = h.clone();
      for _ in 1..=800 {
         if e_hora_de_atualizar(h, ua, 10) {
            // mostrando.
            println!("em {} [ATUALIZADO]", h);
            // diferença de dias decorridos.
            dbg!(h.to_days() - ua.to_days());
            // última atualização registrada.
            ua = h;
         } else { println!("em {} [NADA]", h) };
         // decorrendo os dias.
         h = h.next_date();
         // remove variável ambiente ao fim do programa.
         env::remove_var("ATUALIZADO_HOJE");
      }

      /* "acessando" o período, então a atualização será
       * praticamente instantânea; depois de algum 
       * período, serão "acessados de 2 em 2 dias", e
       * posteriormente diariamente, assim voltando a 
       * atualização para a taxa normal que é atualização
       * semanal. */
      let mut h = DateTuple::new(1981, 03, 15).unwrap();
      let mut ua = h.clone();
      for x in 1..=60 {
         if e_hora_de_atualizar(h, ua, 7) {
            // mostrando.
            println!("em {} [ATUALIZADO]", h);
            // diferença de dias decorridos.
            let _d = dbg!(h.to_days() - ua.to_days());
            // última atualização registrada.
            ua = h;
         } else { println!("em {} [NADA]", h) };
         // decorrendo os dias.
         // dias sem acesso, aumentando.
         if x < 23 
            { h.add_days(x/2); }
         // um dia "acessa", outro não.
         else if x >= 23 && x <= 35 
            { h.add_days(2); }
         else { h.add_days(1); }
         // remove variável ambiente ao fim do programa.
         env::remove_var("ATUALIZADO_HOJE");
      }
      // verificado manualmente pelo "output".
      assert!(true);
   }

   #[test]
   fn testa_atualiza_xmls() {
      // trinta ciclos, então 10seg de testes.
      for _ in 1..=30 {
         atualiza_xmls();
         thread::sleep(Duration::from_secs_f32(0.3));
      }
      assert!(false);
   }

   /* cópia do trecho da função, pois ela pega
    * dados e trabalha ne de maneira intera. Para
    * executa o teste, precisamos mudar tais valores
    * para ficarem favoráveis a testes, se não
    * recebem argumentos destes dados, não é possível
    * trabalhar com eles.
    */
   fn trecho_da_funcao_atualiza_xmls(hoje:&mut DateTuple, 
   ultima:&mut DateTuple) {
      // o rítmo de dias para cada atualização(17 dias).
      let ritmo = 17;
      // criando comando para executar o script:
      // formando caminho ...
      let mut caminho:PathBuf = PathBuf::new();
      caminho.push(env!("RUST_CODES"));
      caminho.push("personalização");
      caminho.push("extern_lib");
      caminho.push("slide_background");
      caminho.push("atualiza_configuracao_xml.py");
      // formando o comando ...
      let  comando:xshell::Cmd = {
         xshell::Cmd::new("python3")
         .arg(caminho.as_os_str())
         .echo_cmd(false)
         .ignore_stdout()
      };

      /* roda o sinal e informa o resultado, se
       * a função dá o sinal. */
      if e_hora_de_atualizar(hoje.clone(), ultima.clone(), ritmo)  { 
         print!("atualizando ... ");
         // roda comando ...
         match comando.run() {
            Ok(_) => {
               println!("feito.");
               // grava data de alteração.
               *ultima = *hoje;
            },
            Err(_) =>
               { println!("um ERROR ocorreu ao executar comando!"); }
         }
      } 
      else 
         { println!("ATUALIZAÇÃO não autorizada!"); }
   }

   #[test]
   #[allow(non_snake_case)]
   fn testa_FAxmls() {
      // lê último data de atualização do BD.
      let mut ultima = DateTuple::new(1945, 06, 06).unwrap();
      // obtem a data de hoje.
      let mut hoje = ultima.next_date();

      /* faz isso simulando executando 60 vezes.
       * onde cada ciclo representa um dia. */
      for _ in 1..=60 {
         // executa trecho.
         trecho_da_funcao_atualiza_xmls(&mut hoje, &mut ultima);
         // pausa para visualizar "saída".
         thread::sleep(Duration::from_secs_f32(0.3));
         // avança um dia.
         hoje.add_days(1);
      }
      // verificação manual feita.
      assert!(true);
   }
}
