// Recursivite indirecte sur calcul de parite
#include <stdio.h>
void pair(int a);
void impair(int b);

// Procedure verifiant la parite
void pair(int a)
{
    if (a % 2 == 0)
    {
        printf("Ce nombre est pair !\n");
    }
    else
        impair(a);
};

// Procedure verifiant l'imparite
void impair(int b)
{
    if (b % 2 != 0)
    {
        printf("Ce nombre est impair !\n");
    }
    else
        pair(b);
}

// Fonction main appelant soit la procedure "pair" ou "impair"
int main()
{
    int nombre;
    printf("Entrer le nombre : ");
    scanf("%d",&nombre);
    if (nombre <= 0)
    {
        printf("Le modulo de ce nombre est impossible !\n");
    } else {
        pair(nombre);
    }
}