/*
   Como a datas especiais, que por serem especiais, ganham exceção na 
 seleções no dia que são marcadas, como dias que os antecedem.

   Para marcar alguma é preciso a data de início, como o dia final(que aqui 
 é a data especial em sí). O nome do arquivo é o 'namespace', se ele não 
 existe na busca, não será selecionado. O nome da 'variável' têm que ser o 
 nome do arquivo XML, e tem que está antes do sinal de igual, já o 
 'intervalo', deve está à frente do nome da variável(que é o mesmo nome do 
 arquivo XML, sem a extensão), separado por dois pontos. A sintaxe dele é 
 uma primeira data que vai até outra( sendo está a data especial em sí) 
 ambas na forma de calendário, mas sem o ano explicitado; sendo ambas 
 separadas pela preposição 'à' onde cada declaração desta estará em sua 
 devida linha. Obviamente elas não podém ser trocadas de ordem, se forem o 
 programa vai detectar, e apitar erro.

   Dias das 'datas especiais' não podem se sobrepor, mais os intervalos 
 cruzados o podem.

   Tudo isso têm que está no arquivo, localizado na raíz rotulado como 
 'data_especiais.conf'; abaixo do cabeçalho "Datas Especiais". O arquivo 
 futuramente, tanto mudará de nome, para algo mais genérico como 
 "configuração.conf", pois, como se pode ver no cabeçalho, servirá como 
 configuração geral do programa, então não será colocado apenas o tal 
 cabeçalho citado lá.
 */

// Biblioteca padrão:
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use std::path::{PathBuf};
// Próprio projeto:
use super::{parte_ii, RAIZ, percentual, avalia_booleano};
use crate::configuracao::coleta_datas_especiais_ii;
// Biblioteca externa:
use date_time::date_tuple::DateTuple;

// melhora codificação e legibilidade:
type Cabecalho = HashMap<String, Vec<String>>;
type LinhaData = (String, DateTuple, DateTuple);
/* array com formatação final da linha das 
 * 'Datas Especiais'. Veja a estrutura 
 * 'LinhaData' para compreede-la bem. */
type DEs = Option<Vec<LinhaData>>;
/* o primeiro elemento é o ínicio, o segundo é
 * o último, ou a data de feriádo na maioria dos
 * casos aqui. */
type IntervaloData = (DateTuple, DateTuple);
// melhorar a legibilidade.
type ID = IntervaloData;


/* Separa um cabeçalho, e as linhas ligadas à ele,
 * por meio de um dicionário. */
fn todas_configuracoes(conteudo: String) -> Cabecalho {
   let mut cabecalho_detectado = false;
   /* Dicionário que com 'chave de cabeçalho', coleta linhas baixo dele. */
   let mut mapa = Cabecalho::new();
   let mut chave: String = "".to_string();

   for l in conteudo.lines().filter(|s| !s.contains(&"#")) {
      // se char um novo cabeçalho.
      if l.contains(&"[") && l.contains(&"]") 
         { cabecalho_detectado = true; } 
      // acessa atual chave.
      else { 
         /* reduzindo impressões desnecessárias
          * por não serem, obviamente, chaves. */
         if chave == ""
            { continue; }
         match mapa.get_mut(&chave) {
            Some(atual) => { 
               let linha = {
                  l.trim_matches('\r')
                  .trim_matches('\t')
                  .trim()
                  .to_string()
               };
               if linha != ""
                  { atual.push(linha); }
            } None => { 
               println!("chave '{}' não existe, provavelmente espaço vázio.", chave); }
         };
      }

      /* extrai conteúdo do cabeçalho, e cria
       * uma chave no dicionário com ela. Isso
       * aciona uma vez por cabeçalho. */
      if cabecalho_detectado {
         let i = l.find('[').unwrap() + 1;
         let f = l.find(']').unwrap();
         let cabecalho = l.get(i..f).unwrap().to_string();
         // mudando nova chave.
         chave = cabecalho.clone();
         mapa.insert(cabecalho, vec![]);
         // desativa até um próximo cabeçalho.
         cabecalho_detectado = false;
      }
   }

   return mapa;
}

/* Pega slice-string formatando uma data(dia e mês) e a transforma nesta 
 especificamente baseado na estrutura adequada, desconsidera o ano, e 
 coloca sempre como o atual. */
fn extrai_data(string: &str) -> DateTuple {
   let partes = string.split_once("/").unwrap();
   let mes = u8::from_str(partes.1.trim()).unwrap();
   let dia = u8::from_str(partes.0.trim()).unwrap();
   let atual = DateTuple::today();
   DateTuple::new(atual.get_year(), mes, dia).unwrap()
}
/* Parse linha contendo nome do arquivo, e período de atual. */
fn parse_linha(linha: String) -> LinhaData {
   let partes = linha.split_once(':').unwrap();

   let mut arquivo_xml = {
      partes.0.trim()
      .to_string()
   };
   arquivo_xml.push_str(".xml");

   // pegando a data e, extraindo em duas.
   let subpartes = partes.1.split_once("à").unwrap();
   let data_inicio = extrai_data(subpartes.0);
   let data_fim = extrai_data(subpartes.1);

   return (arquivo_xml, data_inicio, data_fim);
}

/* Pega do mapa apenas as 'datas especiais', entregando já formatada suas 
linhas; e claro que se não houver tanto o cabeçalho, quanto nada anexado à 
ele, simplesmente "nada" será retornado. */
#[allow(dead_code)]
pub fn coleta_datas_especiais(conteudo: String) -> DEs  {
   let mapa = todas_configuracoes(conteudo);
   let chave = String::from("Datas Especiais");
   
   // se não houver a chave, já rejeita.
   if !mapa.contains_key(&chave)
      { return None; }

   let mut lista_des: Vec<LinhaData> = vec![];
   // se a lista está vázia, também a rejeita.
   if mapa.get(&chave).unwrap().is_empty()
      { return None; }

   // pegando e transformando strings nas devidas estruturas.
   for s in mapa.get(&chave).unwrap().iter()
      { lista_des.push(parse_linha(s.to_string())); }

   return Some(lista_des);
}

/* Faz uma correção no 'DateTuple', em especificamente
 * o ano, para propósitos nos cálculos da codificação. */
fn corrige_dt(hoje: DateTuple, periodo: ID) -> (DateTuple, ID) {
   let ano = periodo.0.get_year();
   let (mes, dia) = (hoje.get_month(), hoje.get_date());
   let novo = match DateTuple::new(ano,mes, dia) {
      Ok(data) => data,
      Err(_) => 
         // cuidando dos anos bissextos.
         { DateTuple::new(ano, mes, dia-1).unwrap() }
   };
   return (novo, periodo);
}
/* verifica se uma determinada data está dentre
 * o 'intervalo de datas' dado. Ignorando o ano,
 * olhando apenas para o dia e mês. */
fn esta_dentro(data: DateTuple, periodo: ID) -> bool {
   let (data, periodo) = corrige_dt(data, periodo);
   data >= periodo.0 && data <=periodo.1
}
/* verifica se, à partir da data dada(obviamente
 * seria hoje), está no período de algum feriado.
 * A array com feriádos também tem que ser cedido.
 */
pub fn e_periodo_de_ferias(data: DateTuple, feriados: DEs) -> bool {
   match feriados {
      Some(mut array) => {
         for tupla in array.drain(..) {
            let intervalo = (tupla.1, tupla.2);
            if esta_dentro(data.clone(), intervalo) {
               //println!("está no feriado '{}' em {}", tupla.0, data);
               return true;
            }
         }
         /* se chegar até aqui, então não está em
          * nenhum feriádo específico, ou perto
          * dele. */
         false
      } None => false
   }
}

/* Faz uma seleção levando transições de datas especiais em consideração 
 * na seleção. Usa a função acima em consideração na seleção randômica.  
 */
#[allow(non_snake_case)]
pub fn parteIII(hoje:DateTuple) -> PathBuf {
   /* Extraindo feriados do arquivo de configuração. */
   let feriados = coleta_datas_especiais_ii().unwrap();
   // obtem uma transição antes.
   let transicao = parte_ii();
   // adicionando raíz ...
   let mut caminho:PathBuf = PathBuf::new();
   caminho.push(RAIZ);

   /* tanto se foi confirmado para algum feriado, 
    * quanto para um possível 'periódo' que foi
    * capturado no bloco. */
   let mut valor_logico = false;
   let mut periodo: Option<IntervaloData> = None;
   /* se estiver em um período configurado, então 
    * um caminho "pode" ser desparado, não é garantido
    * já que é algo probabilístico baseado no 
    * restante do período. */
   if e_periodo_de_ferias(hoje.clone(), Some(feriados.clone())) { 
      // descobrindo que feriado é...
      for t in feriados.iter() {
         let (nome, i) = (t.0.clone(), (t.1.clone(), t.2.clone()));

         /* capturando valor, já que será também usado
          * fora do 'loop', para vê se tal data está 
          * dentro do período de feriado. */
         valor_logico = esta_dentro(hoje.clone(), i.clone()) ;
         /* se estiver dentro, então monta o caminho, também
          * registra o atual 'período' que foi validado. */
         if valor_logico {
            // registrando último período...
            periodo = Some(i);
            // cria caminho.
            match nome.strip_suffix(".xml") {
               Some(resto) => 
                  { caminho.push(resto); }
               None =>
                  { panic!("[ERRO]não possui a extensão."); }
            };
            caminho.push(nome);
            /* abandona o loop no primeiro que achar. Isso,
             * pode não parecer ter muito implicancia ao todo,
             * porém diz algo; se o laço é quebrado, então 
             * com configurações que sobrepõem-se com seus
             * períodos, o primeiro sempre será acionado. */
            break;
         }
      }

      /* é permitido retornar o caminho criado? */
      if let Some(i) = periodo {
         // correção do ano.
         let (hoje, i) = corrige_dt(hoje, i);
         if let Some(p) = percentual(hoje, i) {
            if avalia_booleano(p, valor_logico)
               { return caminho; }
         }
      }
   }
   /* se chegar até aqui, então quer dizer que nenhum 'feriado' 
    * configurado foi acionado, ou, ainda não atingiu sua probabilidade 
    * contínua de seleção. Foi tirado da cláusula 'else', pois o bloco do 
    * 'if' acionado, não garante um retorno construído inteiramente lá. */
   let mut nova_transicao = transicao;
   let mut nome: &str = {
      nova_transicao
      .as_path().file_name().unwrap()
      .to_str().unwrap()
   };

   let exclusao = BTreeSet::<String>::from_iter(
      feriados.iter()
      .map(|t| t.0.to_string())
   );
   /* faz uma nova seleção até ela ser inédita, tirando os feriados. */
   while exclusao.contains(nome) { 
      nova_transicao = parte_ii();
      nome = {
         nova_transicao.as_path()
         .file_name().unwrap()
         .to_str().unwrap()
      };
   }
   // fechado o loop, retorna última transição selecionada.
   nova_transicao
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::fs::{read_to_string};
   use std::time::{Duration, Instant};

   #[test]
   fn saida_sastifatoria() {
      let mut entradas = vec![
         // uma bem simples.
         "\r[Datas Especias]
         \r   corpos_cristes: 05/04 à 23/05
         \r   dia_de_sao_miguel: 11/01 à 12/01
         ".to_string(),
         // bem mais espaçada que a anterior.
         "[DATAS ESPECIAIS]

         corpos_cristes: 05/04 à 23/05
         dia_de_sao_miguel: 11/01 à 12/01



         ".to_string(),
         // vários cabeçalhos e espaçadas.
         "[Configuração]
            hora: 12AM

            data: 13/05
            cor: azul


         [Datas Especias]
            corpos_cristes: 05/04 à 23/05
            dia_de_sao_miguel: 11/01 à 12/01


            dia_do_yourgut: 26/01
         [Diretórios Apropriados]
         pasta1 : '/home/etc'

         pasta2 : '/home/Videos/casas'


         pasta3 : '/root/secreto/muito_secreto'
         ".to_string()
      ];

      for entrada in entradas.drain(..) {
         let resultado = todas_configuracoes(entrada);
         println!("{:#?}", resultado);
      }
   }

   #[test]
   fn transformacao_linhas_datas() {
      let mut entradas = vec![
         "saint_patrick_day: 03/03 à 15/03".to_string(),
         "boogyman: 15/06 à 04/07".to_string(),
         "maria_rosa: 28/10 à 31/10".to_string(),
         // entradas meios desorganizadas nos espaços:
         "dia-de-santa-luzia  : 20/06  à      10/07".to_string(),
         "domingo_sagrado :  12/09       à 18/09".to_string(),
         "   milagra-são-nunca: 11/05  à  23/05".to_string()
      ];

      let atual = DateTuple::today();
      let ano = atual.get_year();

      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("milagra-são-nunca.xml".to_string(), 
           DateTuple::new(ano, 05, 11).unwrap(),
           DateTuple::new(ano, 5, 23).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("domingo_sagrado.xml".to_string(), 
           DateTuple::new(ano, 9, 12).unwrap(),
           DateTuple::new(ano, 9, 18).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("dia-de-santa-luzia.xml".to_string(), 
           DateTuple::new(ano, 06, 20).unwrap(),
           DateTuple::new(ano, 7, 10).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("maria_rosa.xml".to_string(), 
           DateTuple::new(ano, 10, 28).unwrap(),
           DateTuple::new(ano, 10, 31).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("boogyman.xml".to_string(), 
           DateTuple::new(ano, 06, 15).unwrap(),
           DateTuple::new(ano, 07, 4).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("saint_patrick_day.xml".to_string(), 
           DateTuple::new(ano, 3, 3).unwrap(),
           DateTuple::new(ano, 3, 15).unwrap()
         )
      );
   }

   #[test]
   fn coletaDatasEspeciais() {
      let entrada = "[Datas Especiais]

         saint_patrick_day: 03/03 à 15/03
         boogyman: 15/06 à 04/07
         maria_rosa: 28/10 à 31/10
         dia-de-santa-luzia  : 20/06  à      10/07
         domingo_sagrado :  12/09       à 18/09
            milagra-são-nunca: 11/05  à  23/05
      ".to_string();
      let saida = coleta_datas_especiais(entrada);
      assert!(saida.is_some());
      let array = saida.unwrap();
      assert_eq!(array.len(), 6);
      let ano = DateTuple::today().get_year();
      let exemplos = [
         ("milagra-são-nunca.xml".to_string(), 
           DateTuple::new(ano, 05, 11).unwrap(),
           DateTuple::new(ano, 5, 23).unwrap()
         ),
         ("saint_patrick_day.xml".to_string(), 
           DateTuple::new(ano, 3, 3).unwrap(),
           DateTuple::new(ano, 3, 15).unwrap()
         ),
         ("maria_rosa.xml".to_string(), 
           DateTuple::new(ano, 10, 28).unwrap(),
           DateTuple::new(ano, 10, 31).unwrap()
         ),
         ("dia-de-santa-luzia.xml".to_string(), 
           DateTuple::new(ano, 06, 20).unwrap(),
           DateTuple::new(ano, 7, 10).unwrap()
         ),
         ("domingo_sagrado.xml".to_string(), 
           DateTuple::new(ano, 9, 12).unwrap(),
           DateTuple::new(ano, 9, 18).unwrap()
         )
      ];
      for E in exemplos.iter() {
         assert!(array.contains(E));
      }
   }

   #[test]
   fn visualizandoArquivoDeDEs() {
      let caminho = "src/transicao/datas_especiais.conf";
      let conteudo = read_to_string(caminho).unwrap();
      for ld in coleta_datas_especiais(conteudo).unwrap() {
         println!("{:?}", ld);
      }
   }

   #[test]
   fn anoNormal_EPF() {
      let entrada = "
               [Datas Especiais]
         4thJuly: 20/06 à 04/07
         independência:  27/08 à 07/09
         festas-juninas: 11/05  à  23/05
         black-friday : 20/11 à 30/11
         dia-dos-pais: 13/06 à 15/06
      ".to_string();
      let mut inicio = DateTuple::new(1904, 3, 1).unwrap();
      let feriados = coleta_datas_especiais(entrada);
      assert_eq!(feriados.as_ref().unwrap().len(), 5);
      let mut confirmacoes = 0;
      for _ in 1..330 {
         // obtendo nova transição.
         if e_periodo_de_ferias(inicio.clone(), feriados.clone())
            { confirmacoes += 1; }
         // avançando dia ...
         inicio = inicio.next_date();
      }
      assert_eq!(confirmacoes, 10+4+13+12+15);
   }

   #[test]
   fn selecaoBaseadoEmDatasComemorativas() {
      let mut inicio = DateTuple::new(1983, 3, 1).unwrap();
      for _ in 1..330 {
         // obtendo nova transição.
         let nt = parteIII(inicio.clone());
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

   use crate::transicao::parte_iii;

   #[test]
   fn comparandoFuncoes_PARTEIII() {
      let mut inicio = DateTuple::new(1903, 1, 10).unwrap();
      let c = Instant::now();
      let a: Duration;
      let mut t = Duration::new(0, 0);

      for _ in 1..500 {
         // obtendo nova transição.
         let _nt = parteIII(inicio.clone());
         // avançando dia ...
         inicio = inicio.next_date();
      }

      a = t;
      t = c.elapsed();

      for _ in 1..500 {
         // obtendo nova transição.
         let _nt = parte_iii(inicio.clone());
         // avançando dia ...
         inicio = inicio.next_date();
      }
      // variação do último marco(exclui necessidade de reset).
      let T = t - a;
      println!("novo:{:?}\nantigo:{:#?}", t, T);
      /* a intenção inicial não era o desempenho, entretanto, se veio 
       * de bônus, então é só lucro. */
      assert!(t < T);
      println!(
         "{:0.2}% do tempo antigo.
         \r{:0.1} vezes mais rápido.",
         (t.as_nanos() as f32 / T.as_nanos() as f32) * 100.0,
         (T.as_nanos() as f32 / t.as_nanos() as f32)
      );
   }
}
