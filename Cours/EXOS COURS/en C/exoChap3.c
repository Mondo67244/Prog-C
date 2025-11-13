// // Recherche sequentielle en C
// #include <stdio.h>
// int main() {
//     int taille, i, nombre, motif = -1, position = -1;
//     printf(" > Entrer la taille du tableau : ");
//     scanf("%d", &taille);
//     int tableau[taille];

//     // Remplissage du tableau
//     for (i = 0; i < taille; i++) {
//         printf(" > Entrer la valeur %d du tableau : ", i + 1);
//         scanf("%d", &tableau[i]);
//     }

//     // Recherche d'un nombre
//     printf(" > Entrer le nombre a rechercher : ");
//     scanf("%d", &nombre);

//     for (i = 0; i < taille; i++) {
//         if (nombre == tableau[i]) {
//             motif = 0;      // trouvé
//             position = i;
//             break;          // on sort de la boucle, plus besoin de continuer
//         }
//     }
//     return -1;

//     if (motif == 0) {
//         printf("Le nombre %d a ete trouve a l'index %d !\n", nombre, position);
//     } else {
//         printf("Le nombre %d n'a pas ete trouve !\n", nombre);
//     }

//     return 0;
// }



// Recherche par dichotomie
// #include <stdio.h>
// // Fonction de recherche dichotomique
// int dichotomie(int tableau[], int taille, int valeur)
// {
//     int debut = 0;
//     int fin = taille - 1;

//     while (debut <= fin)
//     {
//         int milieu = (debut + (fin - debut)) / 2;

//         // Si la valeur est au milieu
//         if (tableau[milieu] == valeur)
//         {
//             return milieu; // Retourne l'indice où la valeur est trouvée
//         }
//         // Si la valeur est plus grande, ignorer la moitié gauche
//         else if (tableau[milieu] < valeur)
//         {
//             debut = milieu + 1;
//         }
//         // Si la valeur est plus petite, ignorer la moitié droite
//         else
//         {
//             fin = milieu - 1;
//         }
//     }
//     return -1; // Retourne -1 si la valeur n'est pas trouvée
// }

// int main()
// {
//     int taille, i, valeur;

//     // Demander la taille du tableau
//     printf("> 1 - Entrer la taille du tableau : ");
//     scanf("%d", &taille);

//     // Vérifier que la taille est valide
//     if (taille <= 0)
//     {
//         printf("La taille du tableau doit être positive !\n");
//         return 1;
//     }

//     int tableau[taille];

//     // Remplir le tableau
//     printf("Entrez les valeurs du tableau (en ordre croissant) :\n");
//     for (i = 0; i < taille; i++)
//     {
//         printf("> 2-%d - Entrer la valeur %d du tableau : ", i, i + 1);
//         scanf("%d", &tableau[i]);
//     }

//     // Vérifier que le tableau est trié
//     for (i = 1; i < taille; i++)
//     {
//         if (tableau[i] < tableau[i - 1])
//         {
//             printf("Erreur : le tableau doit être trié par ordre croissant !\n");
//             return 1;
//         }
//     }

//     // Demander la valeur à rechercher
//     printf("> 3 - Entrer la valeur à rechercher : ");
//     scanf("%d", &valeur);

//     // Appeler la fonction de recherche dichotomique
//     int resultat = dichotomie(tableau, taille, valeur);

//     // Afficher le résultat
//     if (resultat != -1)
//     {
//         printf("La valeur %d a été trouvée à l'indice %d !\n", valeur, resultat);
//     }
//     else
//     {
//         printf("La valeur %d n'a pas été trouvée !\n", valeur);
//     }

//     return 0;
// }

