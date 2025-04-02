arquivo CSV:

procedimentos_principais

co_procedimento | (1,2,3) |
ortopedia = 1

procedimentos_sequenciais

co_procedimento | (1,2,3) |
ortopedia = 1
neurocirugia= 2
oncologia = 3

relacionamentos
id_principal, id_sequencial, tipo_relacao (sequencial, sequencial sequencial)
1 = sequencial
2 = Sequencial:(Compatíveis Seq.)

cabecalhos:

relações:  
co_procedimento,co_procedimento_sequencial,tipo_relacao,portaria

procedimentos_sequenciais:  
co_procedimento,portaria

procedimentos_principais:  
co_procedimento,portaria
