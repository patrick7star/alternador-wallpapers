
"""
Visualizando todos arquivos XML's da pasta 'Imagens'.
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

   # leitura sintática falha em pegar número
   # de imagens lendo o XML.
   if instancia.qtd == 0:
      # lista com imagens do diretório.
      entradas = os.listdir(instancia.caminho)
      # descontabilizando o arquivo XML.
      instancia.qtd = len(entradas) - 1
   ...

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

def total_de_imagens(arquivo_xml):
   instancia = ConfiguracaoXML.instanciar(arquivo_xml)

   # leitura sintática falha em pegar número
   # de imagens lendo o XML.
   if instancia.qtd == 0:
      # lista com imagens do diretório.
      entradas = os.listdir(instancia.caminho)
      # descontabilizando o arquivo XML.
      return len(entradas) - 1
   ...
   return instancia.qtd
...

import unittest
# sem qualquer classe, apenas testas funções.
class Funcoes(unittest.TestCase):
   def testeJaExistente(self):
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
      arquivo_xml = "/usr/share/backgrounds/cosmos/background-1.xml"
      tempo_legivel = tempo_de_apresentacao(arquivo_xml) // 60
      print("\ntempo apresentação:{0:<3}min".format(tempo_legivel))
   ...
   def variacaoPorConsulta(self):
      raiz = os.path.join(os.getenv("HOME"),"Pictures", "*/*.xml")
      for arquivo_xml in glob.glob(raiz, recursive=True):
         print(arquivo_xml.split("/")[-1].removesuffix(".xml"))
         # nome mais curto para facilitar codificação.
         funcao = tempo_de_apresentacao
         for _ in range(5):
            print("." * 4, funcao(arquivo_xml))
      ...
   ...
...

if __name__ == "__main__":
   from sys import argv
   # está usando o suíte de teste do Python:
   e_o_suite_de_testes = (
      ("python" in argv) and 
      ("-m" in argv) and
      ("unittest" in argv)
   )

   if (not e_o_suite_de_testes):
      print(argv)
      # obtém dados vindo do exterior, via 'stdin', 
      # um 'path' para um arquivo XML com dados da 
      # transição, então joga na função que computação 
      # o tempo e retorna via 'stdout'.
      arquivo_xml = filtra_xml(argv)
      if arquivo_xml:
         print(tempo_de_apresentacao(arquivo_xml))
      else:
         print("nada encontrado!")
   else:
      unittest.main()
...
