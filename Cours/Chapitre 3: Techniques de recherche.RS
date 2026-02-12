
                                    CHAPITRE III : Techniques de recherche
     
                                                      I - Introduction :
                                                      ------------------

                                    C'est une technique ou un processus consistant a localiser une donnee specifique 
                                    dans une structure de donnee (tableau, liste chainnee, arbre, graphe).

                                                               0                0 --- 2 -- 3
                                                              / \               |           \
                                                            1    2              1            4
                                                                / \                         / \
                                                                3  4                       5   6
                                                              Arbre                 Graphe

                                                II - Les types de recherches :
                                                ------------------------------

                          Plusieurs types de recherches sont manipulees en structure de donnee et les plus courant sont :

                                                    1 - Recherche sequentielle ( lineaire ):
                        Le principe est le suivant : 
                          * On parcours la structure de donnee (tableau) elements par elements jusqu'a trouver la valeur cherchee ou atteindre la fin du tableau.
                           -> Les conditions sont les suivantes : 
                             - Cette recherche fonctionne sur n'importe quel tableau (trie ou non);
                             - La complexite s'evalue a deux niveaux :
                                ** Dans le meilleur des cas on aura une complexite lineaire O(1){complexite constante} (valeur trouvee au debut du tableau);
                                ** Dans le pire des cas O(n) a la fin du tableau;
                                soit un tableau a 'n' elements ,
                                  le pseudo code est le suivant (Algorithmique) :

                                                      Algorithme Recherche_Sequentielle
                                                      var T: tableau [0 .. n-1] de entier
                                                      x, i: entier
                                                      debut
                                                        Ecrire("Entrez la taille du tableau : ");
                                                        lire(n);
                                                        pour (i allant de 0 a n-1) faire
                                                          Ecrire(" Entrez l'element ",i," du tableau : ");
                                                          lire(T[i]);
                                                        finpour
                                                        Ecrire("Entrez L'element a rechercher : ");
                                                        lire(x);
                                                        pour (i allant de 0 a n-1) faire
                                                            si (x = T[i]) alors
                                                            Retourner i;
                                                            Ecrire("Element existant en position",i);
                                                            finsi
                                                        finpour
                                                        Retourner -1;
                                                        Ecrire("L'element n'existe pas dans le tableau")
                                                      fin

                                                       >> DEVOIR : Traduire en C. Exo chap 1 - 35 <<
    
                                                          2 - Recherche Dichotomique ou Binaire :
                                                          ---------------------------------------

                                          La recherche Binaire est efficace sur les tableaux tries, elle divise recursivement 
                                            le tableau en deux parties et compare l'element recherche avec celui du milieu .
                                                      Exemple :
                                                              [      |      ]   a: partie gauche
                                                              <=====   =====>   m: milieu du tableau
                                                              a      m      b   b: partie droite
                                                  
                                          La condition sinequanone (ideale) oblige que le tableau sois trie. 
                                            La complexite est de l'ordre de " O(log2 n) " mais comme il n'y a pas de complexite 
                                            O(log2 n) alors la complexite est de O(log n). le pseudo code est le suivant : 

                                          * Comme parametre d'entree on a :
                                                  --> Un tableau trie puis la valeur a chercher ( tableau [valeur] )
                                          * Comme parametre de sortie:
                                                  --> La position de l'element rechercher dans le tableau ou -1 
                                                  si l'element n'est pas present dans le tableau

                                      En supposant "n" le nombre d'elements du tableau et "x" l'element a rechercher,
                                                             le pseudo code est le suivant  :
      
                                                            Algorithme : recherche_dichotomique;
                                                            var tableau : [0 -- n-1] de entier;
                                                            var n , x;
                                                            debut 
                                                              tantque (gauche <= droite) faire
                                                                milieu  <--- (gauche + droite)/2;
                                                                  si (tableau[milieu] = x) alors
                                                                  Retourner milieu; // Retourne l'index ou le truc est situe
                                                                    sinon 
                                                                    si (tableau[milieu] < x) alors 
                                                                      gauche <--- milieu + 1;
                                                                    sinon
                                                                      si (tableau[milieu] > x) alors
                                                                        droite <--- milieu - 1;
                                                                      finsi
                                                                    finsi
                                                                  finsi
                                                              fintanque
                                                              Retourner -1;
                                                            fin

                                                       III - Recherche par table de Hashage :
                                                       --------------------------------------
                                                       
       C'est une methode efficace de recherche d'insertion et de suppression d'elements dans une table de donnees. 
       Contrairement aux recherches sequentielles ou Dichotomiques qui necessitent des comparaisons suscessives, le Hashage permet de trouver directement 
                            l'emplacement d'un element grace a une fonction de Hashage s'ecrivant sous la forme : 

                                                  ---> h(cle) = cle mod N (N = taille de table)
      
                                                           1 -  Principe du Hashage : 
                 L'idee du Hashage est d'utiliser une fonction de Hashage "h" qui transforme une cle (ex. : un nom, numero, identifiant) 
                                                       en une adresse dans une table.

                           Exemple: si on souhaite stocker les etudiants identifies par leur matricule , on aura le schema suivant :

                                           Matricule  ---> Fonction de Hashage ---> Adresse dans la table

                Ainsi au lieu de parcourir toute la table on calcule directement l'adresse a laquelle est stockee / recherchee l'element
      
                                                           2 - Vocabulaire de base :
          
                          -------------------------------------------------------------------------------------------------------------------
                          |              Terme               |                               Definition                                     |
                          | ---------------------------------|-------------------------------------------------------------------------------
                          | Cle                              | Valeur permettant d'identifier un element                                    |
                          | ---------------------------------|-------------------------------------------------------------------------------
                          | Fonction de Hashage              | Une fonction calculant une adresse a partir d'une cle                        |
                          | ---------------------------------|-------------------------------------------------------------------------------
                          | Table de Hashage                 | Tableau dans lequel les elements sont stockes selon leurs adresse de Hashage |
                          | ---------------------------------|-------------------------------------------------------------------------------
                          | Collision                        | Situaton dans laquelle deux cle differentes donnent la meme adresse          |                       
                          | ---------------------------------|-------------------------------------------------------------------------------
                          | Resolution de Collision          | Technique utilisee pour resoudre les Collisions                              |
                          -------------------------------------------------------------------------------------------------------------------

                                                        Soit la fonction de Hashage suivante : 
                                                h(x) = x Mod 5, considerons les cles suivantes : 12, 22 ,32

                                                             " h(cle) = cle mod N "

                                                         - Montrons qu'il y a Collision : 

                                              > h (12) = 12 mod 5       > h (22) = 22 mod 5     > h(32) = 32 mod 5
                                                      = 2                        = 2                    = 2
      
                                                            3 - Fonction de Hashage :
                                                        Une bonne fonction de Hashage doit : 
                                                          --> etre rapide a calculer
                                                          --> produire des adresses repartie uniformement dans la table
                                                          --> dependre uniquement de la cle
   
                                                    *Exemple : Supposons N = 10 et la cle ayant une valeur 123
                                                      determiner l'adresse de l'element
                                                          h(123) = 123 mod 10
                                                                = 3

                                                            4 - Gestion des Collisions :
                                  Meme avec une bonne fonction de Hashage il peut arriver que Plusieurs cles produisent la meme adresse , 
                                            c'est ce que l'on appelle "Collision" il existe deux grandes familles de methodes :
          
                                                              a - Methode d'adresse ouverte :
          
            Ici toutes les donnees sont stockees dans la meme table. Lorsque la Collision se produit on cherche la prochaine case libre selon une regle donnee, 
                          on avance d'une case jusqu'a trouver une case libre selon la formule : --> ( h2(cle, i) = (h(cle) + i) mod N ) 

        On peut egalement utiliser une seconde fonction de Hashage en utilisant la formule suivante : --> ( h2(cle,i) = (h1(cle) + i * h2(cle)) mod N )

                                                              b - Methodes de chainage (adressage ferme/hashage ouvert) :
                                  Chaque cases de la table contient une liste chainee d'elements ayants la meme adresse de Hashage :
                                                          --> Si une Collision se produit , l'element est ajoute a la liste correspondante.
                                                          --> Pour rechercher l'element on parcours la liste a cette Adresse

                                      Plusieurs operations fondamentales sont necessaires:

                                                          --> L'insertion : 
                                                            1-  Calculer l'adresse "adr" <-- h(cle) (on affecte le Hashage de la cle);
                                                            2-  Si la case "adr" est libre, inserer l'element;
                                                            3-  Sinon appliquer la methode de resolution des Collisions.

                                                          --> La recherche :
                                                            1- Calculer l'adresse "adr" <-- h(cle);
                                                            2- Si la cle a cette adresse correspond alors : "succes !";
                                                            3- Sinon suivre la methode de resolution jusqu'a trouver et 
                                                                      au cas contraire l'element est absent.
     
                                                            5 - Domaines d'applications :
                                     Plusieurs domaines sont utilises dans le cadre des fonctions de Hashage notamment :

                                                          --> Gestion des bases de donnees;
                                                          --> Table des symboles dans les compilateurs;
                                                          --> Systeme de gestion des fichiers, Moteurs de recherche;

                            NB: Le Hashage est une technique puissante de recherche et de stockage memoire permettant des access directs et rapides. 
                                                    Toute fois sont efficacite depend fortement :

                                                          --> Du choix de la fonction de Hashage
                                                          --> De la methode de resolution de Collisions
                                                          --> Du taux d'occupation de la table

                                                      EXEMPLE - EXERCICE : 
                                                        Objectif : Ecrire un Algorithme complet dont le but est le suivant 
                                                      1- Creer une table d'Hashage d'entiers
                                                      2- Inserer des valeurs
                                                      3- Effectuer une recherche
                                                      


                                               IV - Recherche dans un arbre binaire de recherche :

    Les arbres binaires de recherche sont des structures de donnees arborescentes utilisees pour "organiser" , "Rechercher" , "Inserer" et "Supprimer" des informations de
  maniere rapide et efficace.
    Un arbre binaire de recherche (ABR) permet une recherche plus performante qu'une recherche sequentielle, en exploitant la structure ordonnee de ces noeuds.

                                1 -  Definitions :
                            Un "arbre binaire" est une structure composee de noeuds tels que :

                                                    --> Chaque noeud contient une valeur (cle);
                                                    --> Chaque noeud peut avoir au plus deux sous-arbres (Un sous arbre gauche et un sous arbre droit);
                                  voir au plus deux sous-arbres (Un sous arbre gauche et un sous arbre droit);

                            Un "arbre binaire" est un ABR si il respecte la propriete de recherche suivante :
                              Pour tout noeud "N" on a :
                                                    --> Toutes les valeurs du sous arbre gauche sont inferieure a la cle de "N";
                                                    --> Toutes les valeurs du sous arbre droit sont sont superieure a la cle de "N".

                                          Exemple :        
                                                                    50 <-- cle
                                                                  /    \
                                                        cle --> 30      70 <-- cle
                                                                /  \    /  \
                                                              20   40  60   80 

                                2 - Principe de la recherche dans un arbre binaire de recherche :
                            L'Objectif c'est de trouver un element dans un arbre organise selon la regle du tri binaire.

                         L'idee cle est la suivante :
 
                                --> Si la cle recherchee est = a la cle du noeud courant alors l'element est trouve.   <-- ;
                                --> Si la cle recherchee est < a la cle du noeud courant alors chercher dans le sous arbre gauche "/" ;
                                --> Si elle est superieure alors chercher dans le sous-arbre droit ;

                           NB: Ce principe repose sur une recherche Dichotomique appliquee a une structure arborescente.
  
                                EXEMPLE -EXERCICE : Ecrire un Algorithme pour la recherche dans un arbre binaire

                                            V - Recherche dans un Graphe (Breath first search et Depth first search):
                                              Breath first search (BFS) : recherche en largeur
                                              Depth first search (DFS) : recherche en profondeur

                                        Un graphe est une structure de donnee utilisee pour modeliser un ensemble d'objets (sommets) et un ensemble de liens (arretes)
                                            > sommet : *    *  *
                                                        *   *
                                            > arretes :    /    /  \
                                                        \     \   \

                                            Les graphes sont omnipresents en informatique, on les utilisent en reseau, dans les relations sociales , 
                                            dans les chemins suivis.
                                            Pour explorer ou rechercher les informations dans un graphe on utilise principalement deux methodes : 
                                                --> Le parcours en largeur (BFS);
                                                --> Le parcous en profondeur (DFS).

                                            Ces deux methodes permettent :
                                                * De visiter tous les sommets atteignables a partir d'un sommet donne;
                                                * De determiner l'existence d'un chemin entre deux sommets;
                                                * De rechercher une donnee specifique dans un graphe
      
                                 1 - Definitions :

                                              -> Sommet (noeud) : c'est un element de base du graphe
                                              -> Arrete (arc/trait) : c'est un lien reliant deux sommets
                                              -> Graphe Oriente : c'est un graphe ou les arretes ont un sens  ex:  * ---> *
                                              -> Graphe non Oriente : c'est un graphe ou les arretes n'ont pas de sens ex: * --- * (sans sens de deplacement)
                                              -> Voisin : c'est un sommet relie directement a un autre
                                              -> Degre d'un sommet : nombre d'arretes relies a un sommet ex: A ---- B  A est de degre 03
                                                                                                            | \
                                                                                                            C  D

                                 2 - Representations d'un Graphe :

                                -> Sous forme de liste d'adjacence : ici chaque sommet a une liste de ses voisins

                                    ex:   voir image 
                                -> La matrice d'adjacence : c'est un tableau carre, Supposons "M" cette matrice, I et J les sommets du graphe "G". 
                                                          Nous avons les relations suivantes

                                                        M[i][j] = 1 Si une arete relie I a j
                                                        M[i][j] = 0 Sinon
                                               
                                                 ex:  D'apres l'image ressort la matrice suivante

                                                                      A    B    C    E    T

                                                                  A   0    1    1    1    1

                                                                  B   1    0    0    1    1

                                                                  C   1    0    0    0    1

                                                                  E   1    1    0    0    1

                                                                  T   1    1    1    1    0
    
                                 3 - Table comparatif des methodes de recherche :

                    ---------------------------------------------------------------------------------------------------------------------------------------
                    | Methode            |  donnees triees  | Complexite Moyenne | Avantages                            |  Inconvennients                 |
                    ---------------------------------------------------------------------------------------------------------------------------------------                                                                                                                                      
                    | sequentielle       |  Non             | 0(n)               | Simple a implementer                 | Lente pour les grands ensembles |
                    ---------------------------------------------------------------------------------------------------------------------------------------
                    | Dichotomique       |  Oui             | 0(log n)           | Tres rapide pour les grands tableaux | Necessite un tri prealable      |                                                                                                                              
                    ---------------------------------------------------------------------------------------------------------------------------------------                                                                                                                                      
                    | Table de Hashage   |  Non             | 0(1)               | Ultra rapide                         | Gere mal les Collisions         |                                                                                                                           
                    ---------------------------------------------------------------------------------------------------------------------------------------                                                                                                                                       
                    | Arbre Binaire      |  Non             | 0(log n)           | Rapide                               | Peut devenir desequilibre       |                                                                                                                       
                    ---------------------------------------------------------------------------------------------------------------------------------------                                                                                                                                       
                    | Graphe BFS/DFS     |  Non             | 0(V + E)           | Utiles pour les structures complexes | Plus difficile a implementer    | 
                    ---------------------------------------------------------------------------------------------------------------------------------------

 
                                                Conclusion :

                                                Le choix de la methode de recherche depend de Plusieurs parametres a savoir : 

                                                  --> La structure des donnees (graphe, tableau , etc)
                                                  --> La taille de l'ensemble
                                                  --> La frequence des recherches  
                                                  --> Le besoin en performance                                                                                                                            

