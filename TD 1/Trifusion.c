#include <stdio.h>
//definition de la fonction
void Trifusion(int A[], int T[], int gauche, int droite);
void Fusionner(int A[], int T[], int gauche, int milieu, int droite);

void Trifusion(int A[], int T[], int gauche, int droite) {
    if (gauche >= droite) {
        return;
    }

    int milieu = gauche + (droite - gauche) / 2;
    Trifusion(A, T, gauche, milieu);
    Trifusion(A, T, milieu + 1, droite);
    Fusionner(A, T, gauche, milieu, droite);
}
void Fusionner(int A[], int T[], int gauche, int milieu, int droite) {
    int i = gauche;
    int j = milieu + 1;
    int k = 0;

    // Fusionner les deux moitiés 
    while (i <= milieu && j <= droite) {
        if (A[i] <= A[j]) {
            T[k++] = A[i++];
        } else {
            T[k++] = A[j++];
        }
    }
    while (i <= milieu) {
        T[k++] = A[i++];
    }
    while (j <= droite) {
        T[k++] = A[j++];
    }

    for (int p = 0; p < k; p++) {
        A[gauche + p] = T[p];
    }
}



int main() {
    int N,i;

    printf("Entrer la taille du tableau : ");
    scanf("%d", &N);

    if (N <= 0) {
        printf("Taille invalide\n");
        return -1;
    }

    int A[N];        
    int T[N];        

    // Saisie
    for (i = 0; i < N; i++) {
        printf("Entrez l'element %d : ", i + 1);
        scanf("%d\n", &A[i]);
    }

    // Tri
    Trifusion(A, T, 0, N - 1);

    // Affichage
    printf("Tableau trie :\n");
    for ( i = 0; i < N; i++) {
        printf("%d ", A[i]);
    }
    printf("\n");

    return 0;
}