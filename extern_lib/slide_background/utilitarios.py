
# bibliotecas do Python:
import os.path, os, time, sys, pickle
from sys import argv
from random import shuffle, randint
# meus módulos:
from variaveis import * 


# FUNÇÕES:
def pegando_nome_do_diretorio(caminho):
   """
   se há uma barra, pegar o conteúdo
   posteriormente a última na extremidade
   esquerda.
   """
   o_que_veio = caminho.split('/')
   # se o resultado for um objeto do tipo lista.
   if type(o_que_veio) == list:
      # removendo todos espaços em brancos.
      while o_que_veio.count('') > 0: 
         o_que_veio.remove('')
      return o_que_veio[-1] # último elemento.
   elif type(o_que_veio) == str:
      return caminho
...

def pares(caminho):
   """
   lista contendo conteúdo do diretório.
   filtrando apenas arquivos.
   filtrando apenas imagens.
   """
   dir_conteudo = [a for a in os.listdir(path=caminho) if a.endswith('.png') or a.endswith('.jpg') or a.endswith('.gif') or a.endswith('.jpeg')] 

   # função auxiliar que faz tudo da ordem
   # superior, porém é declarada internamente
   # pois o algoritmo precisa de um complemento.
   def gera_quase_todos_pares(lista):
      t,i = len(lista), 0
      while i <= (t-1)-1:
         yield (lista[i],lista[i+1])
         i+=1 # vai de 0 à t-2.
   todos_pares = list(gera_quase_todos_pares(dir_conteudo))
   # formando último par e o adicionando 
   # manualmente, que é a última imagem
   # ligando a primeiro, já que a função acima
   # não cuida desta parte.
   ultimo_par = dir_conteudo[-1], dir_conteudo[0],
   todos_pares.append(ultimo_par)
   shuffle(todos_pares) # embaralha...
   return todos_pares
...

def minera_pares(conteudo):
   """pega pares que transicionam, à partir dos
      comentários já adicionados.
   """
   # filtrando apenas os comentários.
   todas_linhas = [
      L for L in conteudo.split('\n') 
      if L.startswith('<!--') and L.endswith('-->')
   ]
   # lista contendo todos os pares.
   # lista auxiliar para formar um par
   # que é limpado ao chegar na quantia
   # de dois.
   pares, par = [], []
   for linha in todas_linhas:
      # dividindo string que representa linha
      # pelos espaços em brancos, e sair verificando
      # cada uma das partes.
      for parte in linha.split():
         # proposições:
         # verifica se a parte tem uma extensão 
         # de imagem, pois assim, é provável que 
         # seja uma.
         A1 = parte.endswith('.jpg')
         A2 = parte.endswith('.jpeg')
         A3 = parte.endswith('.png')
         A4 = parte.endswith('.gif')
         # claro, adiciona se tiver.
         if A1 or A2 or A3 or A4: par.append(parte)
         # quando a contagem da lista "par"
         # atingir dois, então atingiu-se um 
         # par, adiciona na lista de pares, e,
         # limpa o "par" para formar um próximo.
         if len(par) == 2:
            pares.append(tuple(par)) # adiciona par formado.
            par.clear() # limpa lista auxiliar.
         ...
      ...
   ...
   # retorna todos pares filtrados, na ordem 
   # que foram achados, agrupando de dois em dois 
   # em cada linha de comentário lida. Partindo
   # do presuposto que o mais a direita significa
   # o arquivo a ser transicionado do mais à esquerda.
   return pares
...

# pega os valore entre duas tags, estajam elas soltas,
# ou, dentro de outras tags.
def filtra_conteudo_tag(tag, conteudo, subtag=None):
   if subtag:
      novo_conteudo = filtra_conteudo_tag(subtag, conteudo, subtag=None)
      return filtra_conteudo_tag(tag, novo_conteudo)
   else:
      # descontando "tag", e o sinal de tag(<>).
      inicio = conteudo.index('<%s>'%tag) + len(tag) + 2
      fim = conteudo.index('</%s>'%tag)
   return conteudo[inicio:fim]
...

def gera_configuracao_xml(caminho):
   """
   gera um arquivo de slide-wallpapers, onde o tempo
   de transição entre imagens, e o tempo de apresentação
   de uma imagem são definidas baseado em variáveis do
   proprío arquivo xml de configuração.
   """
   if os.path.exists(caminho):
      # nome do arquivo XML, e criando o arquivo disso.
      nome = pegando_nome_do_diretorio(caminho)
      arq = open(os.path.join(caminho,nome) + '.xml', mode='w')
      lista = pares(caminho) # gerando pares a transicionar.
      # computando tempo de apresentação ...
      ta = 0
      if len(lista) >= 48:
         # uma partilha de um dia todo pela
         # quantidade total de imagens.
         ta = 24*3600 // len(lista)
      else:
         # algum valor entre 8min à 20min.
         ta = randint(500, 1200)
      ...
      print('raíz selecionada: "%s"' % caminho)
      print('qtd. de imagens contidas: %i' % len(lista))
      print('tempo de apresentação:%3.0f seg'%ta)
      print('arquivo gerado: \n"%s"' % (nome + '.xml'))
      # tag inicial para composição do arquivo de configuração.
      # mostrando transições.
      for (a1, a2) in lista[0:]:
         print('  ⃘%s >> %s' %(a1, a2))
      arq.write('<background>\n')
      # comentário.
      arq.write('\n<!-- está animação começará às oito horas da noite no Halloween -->')
      # marcando tempo de ínicio. Que é 
      # a data/horário de execução de tal 
      # programa.
      dt = time.gmtime() # formatar data/horário.
      arq.write(
         tempo_inicializacao
         .format(
            dt.tm_year,
            dt.tm_mon,
            dt.tm_mday,
            dt.tm_hour,
            dt.tm_min,
            dt.tm_sec
         )
      )
      # marcando cada exibição.
      for (arq1, arq2) in pares(caminho):
         # comentando cada transição.
         arq.write('\n<!-- %s ==> %s -->' % (arq1, arq2))
         arq.write(parte_estatica.format(ta, os.path.join(caminho, arq1)))
         arq.write(
            parte_transicao.format(
               0, os.path.join(caminho, arq1),
               os.path.join(caminho,arq2)
            )
         )
      ...
      arq.write('\n</background>')
      arq.close() # fechando arquivo depois de escrito.
   else:
      print('tal caminho apontado "%s" não existe!' % caminho)
...

# carrega XML ...
class ConfiguracaoXML:
   """
   classe para armazenar configurações geradas.

   legenda de auxílio para os parâmetros:
       dh - Data/Horário; um objeto time_struct.
       ta - Tempo de Apresentação de uma imagem.
       tt - Tempo de Transição entre imagens.
       pa - Pares de Alternância.
   """
   def __init__(self, caminho, dh, ta,tt,pa):
      # caminho até diretório com imagens.
      self.caminho = caminho
      # struct_time para formar data exata de criação.
      self.data_horario = dh
      # nome obviamente do arquivo xml que configura
      # o slideshow.
      self.nome = pegando_nome_do_diretorio(caminho) + '.xml'
      # guarda a quantia de imagens ao criar tal arquivo.
      self.qtd = len(pa) 
      # tempo de apresentação do papél de parede.
      self.tempo = ta
      # tempo que leva para transicionar de uma imagem
      # para outra.
      self.tempo_alternancia = tt
      # Corrente de transição. Uma lista com tuplas
      # com as imagens que irão se alternar.
      self.corrente = pa
   ...

   # mostra informações sobre o arquivo.
   def __str__(self):
      # cria sequência "gráfica"(numa string) de transição.
      def str_transicao(lista):
         string,meio = '',0
         for item in lista: 
            # reduzindo strin se necessário.
            s = item[0]
            if len(s) > 13: 
               s='"%s"'%(s[0:15]+'(...)'+s[-6:])
               meio = int(len(s)/2)
            string += s +'\n%s🠗\n' %(' ' * meio)
         if lista:
            return string + '\"'+lista[0][0]+'\"\n' + '\t%s🠕'%(' ' * meio)
         return "nenhum par."
      ...
      dh = self.data_horario # abreviação para ajudar.
      # kwa - keyword arguments
      kwa = dict(
         h=dh.tm_hour,
         m=dh.tm_min, 
         s=dh.tm_sec,
         dia=dh.tm_mday,
         mes=dh.tm_mon, 
         ano=dh.tm_year
      )
      # pa - positional arguments.
      pa = (
            self.nome, self.caminho, self.qtd, 
            self.tempo,
            self.tempo_alternancia, 
            str_transicao(self.corrente)
      )
      return texto_info.format(*pa,**kwa)
   ...

   # operador de sobrecarga que compara se outro arquivo
   # deste tipo é igual a este.
   def __eq__(self, X):
      # proposições:
      A = self.caminho = X.caminho # iguais caminhos?
      # formatação básica-comum:
      formatacao = 'data=%d/%m/%Y horário=%H:%M:%S'
      C = time.strftime(formatacao,self.data_horario) == time.strftime(formatacao,X.data_horario) # iguais período de criação?
      D = self.tempo == X.tempo # iguais tempos de apresentação?
      # iguais tempos de transição?
      E = self.tempo_alternancia == X.tempo_alternancia
      F = self.corrente == X.corrente # pares de transições iguais?
      return A and C and D and E and F
   ...

   # função que cria uma instância, dado o caminho do XML compatível.
   def instanciar(caminho):
      """
      transforma um arquivo de configuração XML, tal
      como é gerado acima, e transforma-o de volta
      numa instância desta classe.
      """
      with open(caminho, mode='r') as arquivo:
         string = arquivo.read()
         arquivo.close()
      ...
      # pegando tempo de ínicio.
      funcao = filtra_conteudo_tag # renomeando função.
      ano = int(funcao('year',string))
      mes = int(funcao('month',string))
      dia = int(funcao('day', string))
      hora = int(funcao('hour', string))
      minuto = int(funcao('minute', string))
      segundo = int(funcao('second',string))

      # pegando tempo de apresentação e transição
      # da imagem.
      Ta = float(funcao('duration', string, subtag='static'))
      Tt = float(funcao('duration', string, subtag='transition'))

      # formando perfil com dados colhidos até então...
      formatacao = '{}/{}/{} {}:{}:{}'.format(dia,mes,ano,hora,minuto,segundo)
      dh = time.strptime(formatacao,'%d/%m/%Y %H:%M:%S')

      # cria arquivo de configuração fazendo
      # uma instância, e, passamando argumentos
      # que já foram anteriormente minerados.
      diretorio = os.path.dirname(caminho) # extrai diretório.
      perfil = ConfiguracaoXML(diretorio,dh,Ta, Tt,minera_pares(string))

      return perfil
   ...
...

