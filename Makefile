VERSAO 	= 1.0.0
CAMINHO  = ../versões/personalizacao

salva:
	tar --exclude=target --exclude=Cargo.lock --exclude=LICENSE \
   --exclude=README.md -cvf $(CAMINHO).v$(VERSAO).tar *
	@echo "Backup da versão $(VERSAO) realizado com sucesso."

backups:
	@echo "\nListagem de todos backups feitos no computador:\n"
	@ls --human-readable --size --sort=time -1 $(CAMINHO)*.tar

release-faster:
	cargo rustc --release -- --extern 'utilitarios=lib/libutilitarios.rlib'
	@echo "Compila usando bibliotecas estáticas já compilada."

script-tests:
	@python3 -m unittest scripts.converte_historico_de_escolhas_para_json.Unitarios.test_lista_das_antigas_escolhas
	@python3 -m unittest scripts.converte_historico_de_escolhas_para_json.Unitarios.caminhos_bem_estabelecidos
