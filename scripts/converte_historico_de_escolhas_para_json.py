import json
from pathlib import (Path)
from typing import (List)
from unittest import (TestLoader, TestCase, skip)

FONTE_TXT = Path("data/historico_de_escolhas_feitas.txt")
DESTINO_JSON = Path(FONTE_TXT.parent, "historico_de_escolhas_feitas.json")


def lista_das_antigas_escolhas() -> List[Path]:
	output = []
	NADA_LIDO_MAIS = ""

	with open(FONTE_TXT, "rt") as streaming:
		selecao = streaming.readline()

		while selecao != NADA_LIDO_MAIS:
			output.append(selecao)
			selecao = streaming.readline()

	# Retorna o conteúdo coletado.
	return output

def cria_novo_arquivo_em_json(Input: List[Path]) -> None:
	RECUO = " " * 5

	with open(DESTINO_JSON, "wt", encoding="latin1") as streaming:
		json.dump(Input, streaming, indent=5, sort_keys=True)


class UnitariosI(TestCase):
	def test_lista_das_antigas_escolhas(self):
		conteudo = lista_das_antigas_escolhas()

		print("Total de linhas filtradas: {}".format(len(conteudo)))

		for (k, X) in enumerate(conteudo):
			if k < 7 or (k >= (len(conteudo) - 7)):
				print("{:>5}°. {}".format(k, X.rstrip('\n')))
			else:
				continue

class UnitariosII(TestCase):
	def caminhos_bem_estabelecidos(self):
		print(
			"\n\t\b\b\bDESTINO_JSON: %s\n\t\b\b\bFONTE_TXT: %s"
			% (DESTINO_JSON, FONTE_TXT)
		)

class UnitariosIII(TestCase):
	@skip("Muda parte do banco de dados.")
	def conversao_do_antigo_para_o_novo(self):
		self.assertFalse(DESTINO_JSON.exists())

		lista = lista_das_antigas_escolhas()

		cria_novo_arquivo_em_json(lista)
		self.assertTrue(DESTINO_JSON.exists())

		tamanho = DESTINO_JSON.stat().st_size
		print("{} bytes foram reescritos.".format(tamanho))

		DESTINO_JSON.unlink()
		self.assertTrue(not DESTINO_JSON.exists())
		print("Removido, pois foi criado apanas para verificação.")


if __name__ == "__main__":
	print("Arquivo '%s'já existe? %s" % (DESTINO_JSON.name, DESTINO_JSON.exists()))
	print("Arquivo '%s' existe?  %s" % (FONTE_TXT.name, FONTE_TXT.exists()))
	assert (FONTE_TXT.exists())
	cria_novo_arquivo_em_json(lista_das_antigas_escolhas())

