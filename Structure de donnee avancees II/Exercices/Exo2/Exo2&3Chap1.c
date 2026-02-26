//& donne l'adresse memoire de la variable afin qu'on puisse y stocker
//des valeurs.

//fgets = File Get String qui permet de lire une ligne entiere

//contenu c'est le tampon dans lequel on stocke la ligne entree
//par l'utilisateur

//100 c'est la taille maximale a lire pour eviter les depassements de memoire

//stdin indique qu'il faut lire depuis le clavier

//fprintf = File Print Formatted qui permet d'ecrire une ligne dans un fichier
//ouvert au prealable

#include <stdio.h>
int main(){
    //Exo 2 : creer le fichier examen.txt et y ecrire quelques lignes de texte
    int i , n;
    char contenu[100];
    //Ouverture du fichier
    FILE *fichier = fopen("Examen.txt","w"); //w ecrase tout
    if (fichier == NULL){
        perror("Erreur d'ouverture\n");
        return 1; //Execution avec Echec
    }
    printf("Entrer le nombre de phrase : ");
    scanf("%d",&n);
    getchar();
    //Ecriture de quelque lignes de texte
    for (i = 0; i < n; i++){
        printf("Entrez la phrase %d : ", i +1);
        fgets(contenu, 100, stdin);/*ecrire dans le tampon contenu en limit 100 et 
    l'imput du clavier*/
        fprintf(fichier,"%s",contenu);//ecrire dans le 
    }
    printf("le fichier a ete cree avec success\n");
    fclose(fichier); //Fermeture du fichier
    //Exo 3: Lecture du contenu du fichier et affichage a l'ecran
    fichier = fopen("Examen.txt", "r");
    if (fichier == NULL){
        perror("Erreur d'ouverture\n");
        return 1; //Execution avec Echec
    }
    while (fgets(contenu, 100, fichier) != NULL){
       printf("J'ai lu : %s",contenu);
    }
    fclose(fichier);

    //Exo 5: supprimer le fichier examen.t
    remove("Examen.txt");
    return 0;
}