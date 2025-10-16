// >> Exercice 1:
// //Une procedure qui affiche un messsage de bienvenue
// #include <stdio.h>
// void bienvenue(){
//     printf("Bienvenue sur ma procedure en C!\n");
// };
// int main(){
//     bienvenue();
//     return 0;
// }

/// >> Exercice 2:
// //Fonction qui calcule la  somme de deux entiers
// #include <stdio.h>
// int somme(int a, int b){
//     int s;
//     s = a+b;
//     printf(" La somme %d + %d donne %d\n", a, b, s);
//     return s;
// }
// int main(){
//     int a, b;
//     printf("Entrer le premier nombre: ");
//     scanf("%d",&a);
//     printf("Entrer le deuxieme nombre: ");
//     scanf("%d",&b);
//     somme(a,b);
//     return 0;
// }

/// >> Exercice 3:
//Procedure qui affiche le carre d'un entier passer en parametre
// #include <stdio.h>
// void carre(int a){
//     int s;
//     s = a*a;
//     printf("Vous avez choisi le chiffre %d ! son carre est : %d!",a,s);
// };
// int main(){
//     carre(5);
// }

/// >> Exercice 4:
//Fonction trouvant l'entier le plus grand
// #include <stdio.h>
// int maximum(int a,int b){
//     int max;
//     if (b > a){
//         printf("Le maximum est : %d\n",b);
//         max = b;
//     } else if (a > b){
//         printf("Le maximum est : %d\n",a);
//         max = a;
//     } else if (a == b){
//         printf("Le nombre %d et %d sont similaires !\n",a,b);
//         max = 0;
//     } else {
//         printf("Verifier la valeur saisie et reessayer");
//         max = 0;
//     }
//     return max;
// }
// int main(){
//     // int c,d;
//     // printf("Vous aller entrer deux entier pour savoir qui est le plus grand !\n");
//     // printf("Entre le nombre aleatoire 1: ");
//     // scanf("%d",&c);
//     // printf("Entre le nombre aleatoire 2: ");
//     // scanf("%d",&d);
//     maximum(67, 65);
//     return 0;
// }

/// >> Exercice 5:
// Factoriel recursif
#include <stdio.h>
int factorecursif1(int nombre){
if (nombre == 0 || nombre == 1){
    return 1;
} else {
    int resultat = nombre * factorecursif1(nombre - 1);
    printf("Le factoriel de %d est : %d\n", nombre,resultat);
    return resultat;
}
}
int main(){
    factorecursif1(5);
}



// >> Exercice 6:
//Factoriel iteratif

// int factorecursif(int nombre)
// {
//     int i, final = 1;
//     for (i = 1; i < nombre + 1; i++)
//     {
//             final = final * i;

//         if (nombre < 0)
//         {
//             printf("Le factoriel d'un nombre negatif est impossible.");
//         }
//         else
//         {
//             printf("%d x ", i);
//             printf("\n>> Le factoriel de %d est %d !\n", nombre, final);
//         }
//     }
//     return final;
// }
// int main()
// {
//     factorecursif(-9);
//     return 0;
// }