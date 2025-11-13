


//Algorithme tri par insertion
#include <stdio.h>
int main(){
    int i,j,x,n;
    //On defini la taille du tableau
    printf("Entrer la taille du tableau : ");
    scanf("%d",&n);
    int tableau[n];
    for (i = 0 ; i < n; i++){
    //Remplissage du tableau
        printf("Entrer la valeur %d du tableau: ",i+1);
        scanf("%d",&tableau[i]);
    }
    for (i =0 ; i < n ; i++){
        x = tableau[i];
        j = i - 1;
        while(j > 0 && tableau[j] > x){
            tableau[j+1] = tableau[j];
            j = j - 1;
        }
        tableau[j+1] < x;

    }
    printf("Tableau trie : \n");
    for (i=0;i<n;i++){
    printf("%d",tableau[i]);
    }
    return 0;
}