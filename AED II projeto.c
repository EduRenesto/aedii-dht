#include <stdio.h>
#include <stdlib.h>
#include <curses.h>
#include <iostream>
#include <string.h>

int indice(char temp[30]);
#define max 5 // número máximo de dados salvos na tabela
#define letras 25  // número de letras consideradas do alfabeto,
 									 // que vai resultar no tamanho da tabela.

struct tabela
{
	char info[30]; // informação armazenada no nó
	int ip; // simular endereço de ip do nó
} info_tabela[letras];

int main(void){
	int i;
	int j;
	short ip_aux = 0;
	int flag;
	char aux[30];

	//Recebe os valores digitados
	printf("Tabela Hash que armazena informação + ip \n");
	printf("Nao ha tratamento de colisoes \n");

	for (i=0;i<letras;i++) // inicializacao
	{
		strcpy (info_tabela[i].info,"ainda nao prenchido");
		info_tabela[i].ip = 0;
	}

	printf("\nDigite a informacao a ser armazenada,\n");
	printf("Maximo de 30 caracteres\n");
	for (i=0;i<max;i++)
	{
		printf("Entre com informacao %d: ",i+1);
		scanf("\n %s", aux);
		j = indice(aux);
		strcpy(info_tabela[j].info,aux);
		info_tabela[j].ip = rand();
		printf("\n");
	}

	//flag que permite pesquisar informação desejada na tabela Hash
	flag = 1;
	while (flag ==1)
	{
		printf("\n");
		printf("Escreva qual informacao deseja saber o endereco \n");
		scanf("\n %s",aux);
		j = indice(aux);
		printf("\n");
		printf("Endereco da informacao eh: %d ", info_tabela[j].ip);
		printf("\n");
		printf("Indice na tabela hash eh: %d ", j);
		printf("\n");
		printf("Digite S para uma nova pesquisa ou qualquer outra tecla para sair \n");
		scanf("\n %s", aux);
		if (aux[0] =='S') 
			flag = 1;
		else 
			flag = 0;
		printf("\n");
	}
}

int indice(char *temp)
{
	return (temp[0]-97);
}
