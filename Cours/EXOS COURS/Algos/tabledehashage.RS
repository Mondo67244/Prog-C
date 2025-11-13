// Algorithme Recherche_Hashee;
// But : Rechercher une valeur dans un tableau en utilisant une table de hachage

Variables
    tableau, table_hash : tableau entier;
    taille, i, valeur, hash : entier;

Début
    Écrire("Entrer la taille du tableau : ");
    Lire(taille);

    Dimensionner tableau[0..taille-1];
    Dimensionner table_hash[0..taille-1];

    // Remplissage du tableau et calcul du hachage pour chaque élément
    Pour i ← 0 à taille - 1 faire
        Écrire("Entrer la valeur ", i, " du tableau : ");
        Lire(tableau[i]);
        table_hash[i] ← (tableau[i] % taille);
    FinPour

    // Recherche d'une valeur
    Écrire("Entrer la valeur à rechercher : ");
    Lire(valeur);
    hash ← (valeur % taille);

    // Verification de la valeur hashe dans le tableau
    i ← 0
    TantQue i < taille ET table_hash[i] ≠ hash faire
        i ← i + 1
        Si i < taille Alors
            Si tableau[i] = valeur Alors
                Écrire("La valeur ", valeur, " a été trouvée à l’indice ", i)
            Sinon
                Écrire("Collision détectée : même hash mais valeur différente.")
            FinSi
        Sinon
            Écrire("La valeur ", valeur, " n’a pas été trouvée.")
        FinSi
    FinTantQue
    
Fin