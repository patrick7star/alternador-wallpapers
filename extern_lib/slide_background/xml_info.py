
"""
Visualizando todos arquivos XML's da pasta
"Imagens"
"""

import os, os.path, glob
from utilitarios import ConfiguracaoXML


def info_importante(instancia):
   quantia = instancia.qtd 
   tempo = instancia.tempo
   if __debug__:
      print(tempo)
   print("""{3}
      \rqtd. de imagens: {0}
      \rtempo de apresentação: {1}min
      \rapresentação se dará em: {2}h 
      """
      .format(
         quantia,
         tempo//60,
         quantia*tempo//3600,
         instancia.caminho
      )
   )
...

# computa o total de segundos que toda a apresentação durará.
def tempo_de_apresentacao(arquivo_xml):
   instancia = ConfiguracaoXML.instanciar(arquivo_xml)
   return instancia.tempo * instancia.qtd
...

# filtra o arquivo XML dos argumentos:
def filtra_xml(argumentos):
   for a in argumentos:
      if a.endswith(".xml"):
         return a
   ...
   return None
...

if __name__ == "__main__":
   from sys import argv
   if __debug__:
      raiz = os.path.join(os.getenv("HOME"),"Pictures", "*/*.xml")
      for arquivo_xml in glob.glob(raiz, recursive=True):
         instancia = ConfiguracaoXML.instanciar(arquivo_xml)
         info_importante(instancia)
      ...

      caminho = os.path.join(
         os.getenv("HOME"),
         "Pictures/computação/computação.xml"
      )
      instancia = ConfiguracaoXML.instanciar(caminho)
      tempo_total = tempo_de_apresentacao(caminho)
      print(
         "tempo total de apresentação: {} ==> {}h"
         .format(tempo_total, tempo_total // 3_600)
      )
      print(argv)
   else:
      # obtém dados vindo do exterior, via 'stdin', 
      # um 'path' para um arquivo XML com dados da 
      # transição, então joga na função que computação 
      # o tempo e retorna via 'stdout'.
      arquivo_xml = filtra_xml(argv)
      if arquivo_xml:
         print(tempo_de_apresentacao(arquivo_xml))
      else:
         print("nada encontrado!")
   ...
...
