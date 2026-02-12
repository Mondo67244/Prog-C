#include <stdio.h>

// Déclaration de la structure
struct Biscuit {
    char nom[20];
    int poids;
    float prix;
    struct Fabrication{
        int jour;
        int mois;
        int annee;
    }
    struct Peremption {
        int jour;
        int mois;
        int annee;
    } peremption;
};

// On donne un nom au type pour plus de clarté
typedef struct Biscuit Biscuit;
typedef struct Fabrication Fabrication;
typedef struct Peremption Peremption;


// Prototypes de fonctions
void saisirDate(Peremption*p);
void saisirBiscuit(Biscuit *b);
void saisirdatefabrication(Fabrication)

int main() {
    Biscuit b;
    
    saisirBiscuit(&b);

    printf("Biscuit saisi : nom du biscuit : %s\n, poids : %d g\n, prix : %f fcfa\n, date de peremption : %d/%d/%d\n", b.nom, b.poids, b.prix, b.peremption.jour, b.peremption.mois, b.peremption.annee);

    return 0;
}

// Fonction pour saisir un biscuit complet
void saisirBiscuit(Biscuit *b) {
    printf("Entrer le nom du biscuit : ");
    scanf("%s", b->nom);
    printf("Entrer le poids (en g) : ");
    scanf("%d", &b->poids);
    printf("Entrer le prix : ");
    scanf("%f", &b->prix);
    
    
    printf("--- Date de peremption ---\n");
    saisirDate(&b->peremption);
}


// Fonction pour saisir une date
void saisirDate(Peremption *p) {
    printf("Entrer le jour de peremption : ");
    scanf("%d", &p->jour);
    printf("Entrer le mois de peremption : ");
    scanf("%d", &p->mois);
    printf("Entrer l'annee de peremption : ");
    scanf("%d", &p->annee);
}

void saisirdatefabrication(Fabrication *f) {
    printf("Entrer le jour de fabrication : ");
    scanf("%d", &p->jour);
    printf("Entrer le mois de fabrication : ");
    scanf("%d", &p->mois);
    printf("Entrer l'annee de fabrication : ");
    scanf("%d", &p->annee);
}

