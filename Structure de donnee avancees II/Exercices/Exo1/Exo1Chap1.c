#include <stdio.h>
int main(){
    FILE *C, *F;
    int X, i, ro, val, compt,n;
    F = fopen("mondo.txt", "a");
    printf("Entrer le nombre de variables : ");
    scanf("%d",&n);
    for ( i = 0; i < n; i++ ){
        printf("Entrer le nombre ");
        scanf("%d",&X);
        fprintf(F,"%d\n",X);
    } 
    fclose(F);

    F = fopen("mondo.txt","r");
    while (!feof(F))
    {
     fscanf(F,"%d",&X);
     printf("%d\n",X); 
    }
    //EXO 3
    ro = 0;
    F = fopen("mondo.txt", "r");
    while (!feof(F))
    {
        fscanf(F,"%d", &X);
        ro = ro + X;
    }
    fclose(F);
    //EXO 4
    F = fopen("mondo.txt", "r");
    C = fopen("copie.txt","a");
    while (!feof(F))
    {
      fscanf(F,"%d",&X);
      printf("%c",&X);
      fprintf(C,"%d",&val);
    }
    fclose(F);
    fclose(C);
    return 0 ;
}