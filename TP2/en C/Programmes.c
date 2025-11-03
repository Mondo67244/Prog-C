//Programme 1 : // Recursivite directe
// #include<stdio.h>
// int factorecursif(int nombre){
// if (nombre == 0 || nombre == 1){
//     return 1;
// } else {
//     int resultat = nombre * factorecursif(nombre - 1);
//     printf("Le factoriel de %d est : %d\n", nombre,resultat);
//     return resultat;
// }

// }
// int main(){
//     int nombre;
//     printf("Entrer le nombre : ");
//     scanf("%d",&nombre);
//     factorecursif(nombre);
// }

//Programme 2 : // Recursivite directe
// #include <stdio.h>
// int fibo(int nombre){
//     if (nombre <= 0) return 0;
//     else if (nombre == 1) return 1;
//     int resultat = fibo(nombre - 1 ) + fibo(nombre - 2);
//     return resultat;
    
// }
// int main(){
//     int nombre;
//     printf("Entrer le nombre : ");
//     scanf("%d",&nombre);
//     int resultat = fibo(nombre);
//     printf("Fibonnaci(%d) = %d\n", nombre, resultat);
//     return 0;
// }

// //Programme 3 : // Recursivite directe
// #include <stdio.h>
// int npremiers(int nombre){
//     if (nombre == 0) return 0;
//     int resultat = nombre + npremiers(nombre -1);
// }
// int main(){
//     int nombre;
//     printf("Entrer le nombre : ");
//     scanf("%d",&nombre);
//     printf(" les n premiers sont : %d\n",npremiers(nombre));
//     return 0;
// }

// // Programmme 4 : Recherche recursive d'un element dans un tableau
// #include <stdio.h>
// int recherche (int tab[], int taille, int x , int index) {
//     if (taille >= taille) return 0;
//     if (tab[0] == x) return 1;
//     int resultat = recherche(tab , taille, x, index + 1);
//     return resultat;
// }
// int main(){
//     int i , taille, rang, x, motif;
//     printf("Entrer la taille du tableau : ");
//     scanf("%d",&taille);
//     int tab[taille];
//     for (i = 0 ; i < taille ; i++){
//         printf(" Entrer la valeur %d du tableau :",i+1);
//         scanf("%d",&tab[i]);
//     }
//     printf("Entrer l'elememt a rechercher: ");
//     scanf("%d", &x);
//     for (i =0 ; i < taille ; i++){
//         if (tab[i] == x){
//             // rang = i;
//             motif = 0;
//         printf("La valeur %d a ete trouve a l'index %d\n",x, recherche(tab + 1 , taille,x));
//         break;
//         } else {
//         motif = 1;
//         }
//     }
//     if (motif == 1){
//     printf("La valeur n'as pas ete trouvee dans le tableau !");
//     }
//     return 0;
// }

// //Programme 5 : Nombre de chiffres dans un nombre 
// #include <stdio.h>
// int nombrechiffres(int nombre){
//     if (nombre == 0){
//         return 0;
//     }
//     int resultat = 1 + nombrechiffres(nombre /10);
//     return resultat;
// }
// int main(){
//     int nombre,resultat;
//     printf("Entrer le nombre : ");
//     scanf("%d",&nombre);
//     resultat = nombrechiffres(nombre);
//     printf("Le nombre de chiffres de l'entier est : %d",resultat);
// }

//Programme 6 : 