#include <stdio.h>
#include <stdlib.h> //contient les fonctions rand() et srand()
#include <time.h> //recupere le temps de la machine
//scanf lit une valeur d'entree (clavier) et stocke la valeur a l'adresse d'une variable
//fscanf lit une entree (fichier) et stocke le resultat a une adresse

int main(){
    int i, seqv, sequence[100];
    FILE *fichier;
    srand(time(NULL)); //defini la valeur de rand() en fonction du temps de la machine
    printf("Entrer le nombre de nombre de la sequence : ");
    scanf("%d", &seqv);
    //on rempli le tableau de sequences de nombre aleatoires dans la limite voulue
    for (i = 0 ; i < seqv ; i++){
        sequence[i] = rand() % 101;
    }
    //on verifie l'etat du fichier
    fichier = fopen("console.txt", "w");
    if (fichier == NULL){
        perror("Erreur lors de l'ouverture du fichier.");
        return 1;
    }
    //on ecris dans le fichier console.txt
    for (i = 0 ; i < seqv; i++){
        fprintf(fichier, "%d\n", sequence[i]);
    }
    printf("Fichier ecris avec success!");
    fclose(fichier);
    
    //on ouvre de nouveau le fichier en lecture
    fichier = fopen("console.txt","r");

    //affichons le contenu du fichier a l'ecran
    while(!feof(fichier)){
    fscanf(fichier, "%d", &sequence[i]);
    printf("%d\n", sequence[i]);
    }
    fclose(fichier);
    return 0;
}