#include <stdio.h>

int main()
{
    int i, taille, valeur, hash, verifhash, position = -1;
    int collisionAvecAutreValeur = 0;

    // Saisie de la taille du tableau
    printf("Entrer la taille du tableau : ");
    scanf("%d", &taille);

    int tabnormal[taille];
    int tabhash[taille];

    // Remplir le tableau
    for (i = 0; i < taille; i++)
    {
        printf("Entrer la valeur %d du tableau : ", i + 1);
        scanf("%d", &tabnormal[i]);
        tabhash[i] = tabnormal[i] % taille; // hash simple
    }

    // Recherche d'une valeur
    printf("\nEntrer la valeur a rechercher : ");
    scanf("%d", &valeur);

    verifhash = valeur % taille;

    // Recherche dans le tableau
    for (i = 0; i < taille; i++)
    {
        if (tabhash[i] == verifhash && tabnormal[i] == valeur)
        {
            position = i; // valeur trouvée
        }
        else if (tabhash[i] == verifhash && tabnormal[i] != valeur)
        {
            collisionAvecAutreValeur = 1;
        }
    }

    // Résultat de la recherche
    if (position != -1)
    {
        printf("\n Valeur %d trouvee !\n", valeur);
        printf("Son hash : %d\n", verifhash);
        printf("Sa position : %d\n", position);
        if (collisionAvecAutreValeur)
        {
            printf(" Une ou plusieurs autres valeurs du tableau ont le meme hash !\n");
            printf("Cela signifie que %d et d'autres valeurs partagent le meme hash (%d).\n", valeur, verifhash);

            // Optionnel : afficher lesquelles
            printf("Valeurs ayant ce hash : ");
            for (i = 0; i < taille; i++)
            {
                if (tabhash[i] == verifhash)
                    printf("%d ", tabnormal[i]);
            }
            printf("\n");
        }
        else
        {
            printf(" Aucun autre element ne partage ce hash.\n");
        }
    }
    else
    {
        printf("\n Valeur %d non presente dans le tableau.\n", valeur);
        printf("Son hash theorique : %d\n", verifhash);

        if (collisionAvecAutreValeur)
        {
            printf(" Une ou plusieurs autres valeurs du tableau ont le meme hash !\n");
            printf("Cela signifie que %d et d'autres valeurs partagent le meme hash (%d).\n", valeur, verifhash);

            // Optionnel : afficher lesquelles
            printf("Valeurs ayant ce hash : ");
            for (i = 0; i < taille; i++)
            {
                if (tabhash[i] == verifhash)
                    printf("%d ", tabnormal[i]);
            }
            printf("\n");
        }
        else
        {
            printf(" Aucun autre element ne partage ce hash.\n");
        }
    }

    return 0;
}
