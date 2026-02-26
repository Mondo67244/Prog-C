//Ecrivons un programme C qui copie le contenu d'un autre fichier et
// le met dans un autre fichier
#include <stdio.h>
int main(){
    FILE *fichier1, *fichier2;
    char tampon[250];//tableau tampon pour stocker le contenu du fichier 1
    fichier1 = fopen("fichier1.txt","r");
    fichier2 = fopen("fichier2.txt","w");
    if (fichier1 == NULL){
        perror("Erreur lors de l'ouverture du fichier");
        return 1;
    }
    while (!feof(fichier1)){//tant qu'on n'atteint pas la fin du fichier continue de lire
    fgets(tampon, 250, fichier1);
    fprintf(fichier2, "%s", tampon);
    }
    printf("Copie du contenu du fichier1.txt vers fichier2.txt effectuee avec succes!\n");
    fclose(fichier1);
    fclose(fichier2);
    return 0;

}