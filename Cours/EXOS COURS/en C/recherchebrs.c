#include <stdio.h>

#define MAX_NOEUDS 100  // Taille maximale de l'arbre (ajustez si nécessaire)

// Définition de la structure : gauche et droite sont des indices (-1 pour vide)
struct noeud {
    int valeur;
    int gauche;
    int droite;
};

struct noeud arbre[MAX_NOEUDS];  // Tableau statique pour les nœuds
int prochain_libre = 0;  // Indice du prochain nœud libre

// Fonction d'insertion : prend l'indice racine et la valeur à insérer, retourne le nouvel indice racine
int inserer(int racine, int x) {
    if (racine == -1) {
        // Créer un nouveau nœud en utilisant le tableau statique
        if (prochain_libre >= MAX_NOEUDS) {
            printf("Erreur : Arbre plein !\n");
            return -1;
        }
        arbre[prochain_libre].valeur = x;
        arbre[prochain_libre].gauche = -1;
        arbre[prochain_libre].droite = -1;
        prochain_libre++;
        return prochain_libre - 1;  // Retourner l'indice du nouveau nœud
    } else if (arbre[racine].valeur < x) {
        // Insérer à droite si plus grand
        arbre[racine].droite = inserer(arbre[racine].droite, x);
    } else {
        // Insérer à gauche si plus petit ou égal
        arbre[racine].gauche = inserer(arbre[racine].gauche, x);
    }
    return racine;  // Retourner l'indice racine mis à jour
}

// Fonction de recherche : retourne 1 si trouvé, 0 sinon
int rechercher(int racine, int x) {
    if (racine == -1) {
        return 0;  // Non trouvé
    }
    if (arbre[racine].valeur == x) {
        return 1;  // Trouvé
    } else if (arbre[racine].valeur < x) {
        return rechercher(arbre[racine].droite, x);
    } else {
        return rechercher(arbre[racine].gauche, x);
    }
}

int main() {
    int a, n, i, nbre, res;
    printf("Entrer la taille du tableau: ");
    scanf("%d", &n);

    int racine = -1;  // Initialiser l'indice racine à -1 (arbre vide)

    for (i = 0; i < n; i++) {
        printf("Entre l'element %d : ", i);
        scanf("%d", &a);
        racine = inserer(racine, a);  // Mettre à jour la racine après insertion
    }

    printf("Entre l'element a rechercher : ");
    scanf("%d", &nbre);

    res = rechercher(racine, nbre);

    if (res == 1) {
        printf("Nombre %d trouve\n", nbre);
    } else {
        printf("Nombre introuvable !\n");
    }

    return 0;
}