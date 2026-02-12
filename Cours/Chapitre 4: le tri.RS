                                CHAPITRE 4 : TECHNIQUES DE tri

                                    I - Introduction :

                                Le tri est une operation fondamentale en informatique et en Algorithmique car il s'opere dans de nombreuse situation. 
                                Il consiste a reorganiser les elements d'un tableau ou d'une liste selon un ordre defini souvent croissant ou decroissant. 
                                Les techniques de tri sont utilisees dans les domaines suivants :
                                
                                -> Les bases de donnees (classemment des enregistrements);
                                -> Les algorithmes de recherche;
                                -> Les traitements statistiques ou graviques;
                                -> L'optimisation d'access aux informations.

                                    II - Types de tri :

                                Avant d'aborder la notion sur les types de tri nous allons presenter les objectifs. Le tri vise a :

                                -> Faciliter la recherche des informations (acces rapide);
                                -> Organiser les donnees pour une lecture plus claire;
                                -> Optimiser d'autres traitements a savoir :  la "fusion" , les "statistiques" etc;
                                On distingue deux grandes familles de tri: 
                                
                                    * Les tri interne : dans lequel les donnees tiennent entierement en memoire (tri de tableaux en memoire)
                                    * Les tri externes : les donnees sont trop volumineuses et necessitent un stockage externe (disque dur)
                                    * On peut egalement comparer les tris selon un certain nombre de criteres  


                                            -----------------------------------------------------------------------------------
                                            |            Criteres         |        Signification                              | 
                                            -----------------------------------------------------------------------------------
                                            | Complexite temporelle       | Temps d'execution du tri                          |
                                            -----------------------------------------------------------------------------------
                                            | Complexite spatiale         | Memoire utilisee                                  |
                                            -----------------------------------------------------------------------------------
                                            | Stabilite                   | Si deux elements egaux gardent leur ordre initial |
                                            -----------------------------------------------------------------------------------
                                            | Methode                     | Comparatif (avec comparaison) ou non comparatif   |       
                                            -----------------------------------------------------------------------------------
                                            

                                    III - Principales techniques de tri :

                                En Algorithmique il existe plusieurs versions de tri dont les plus communement utilises sont :

                                -> Tri par echange (Tri a bulle/bubble sort, Tri par selection)
                                -> Tri par insertion
                                -> Tri par fusion (merge sort)
                                -> Tri rapide (Quick sort)

                                    1 - bubble sort (Tri a bulle) :

                                    Le Principe du tri a bulle est le suivant :

                                    - Comparer des paires successives d'elements et echanger ceux qui sont dans le mauvais ordre
                                    - A chaque passage , le plus grand element remonte en fin du tableau comme une bulle dans l'eau

                                                    L'algorithme (pseudo - code ) est le suivant : 

                                                    Procedure tribulles (T, n)
                                                    debut
                                                        Pour (i <- 0  a n -1) faire 
                                                        Pour (j <- 0 a n-1) faire 
                                                            si (T[j] > T[j+1]) alors 
                                                            echanger T[j] et T[j + 1]
                                                            finsi
                                                        finpour
                                                        finpour
                                                    finProcedure


                                On peut avoir les complexite suivantes pour cet exemple :

                                -> Dans le pire des cas : O(n^2) (quadratique)
                                -> Dans le meilleur des cas : O(n)
                                -> Dans le moyen des cas la complexite est de  : O(1) (complexite lineaire)
                                    
                                    2 - Le tri par selection :

                                    il consiste a :
                                    -> Chercher le plus petit element de la liste (ou Tableau)
                                    -> Le placer en premiere position de celle-ci 
                                    -> Repeter l'operation sur le reste des elements de la liste

                                Le mecanisme suivant represente les etapes successives d'un tri par selection sur une liste de 06 elements
                                
                                Exemple : 

                                Liste : 101 115 30 63 47 20
                                > 1ere selection: -> 20
                                placement: 20 115 30 63 47 101
                                > 2eme selection : -> 30
                                placement: 20 30 115 63 47 101
                                > 3eme selection: -> 47
                                placement: 20 30 47 63 115 101
                                > 4eme selection: -> 101
                                placement: 20 30 47 63 101 115

                                Remarque : on constate que apres le nieme placement les "n" plus petits elements de la liste sont a leur place
                                definitive et l'algorithme est le suivant: 

                                DEVOIR : IMPLEMENTER CET Algorithme EN C

                                Algorithme tri_selection
                                var T:tableau[0..n-1] d'entiers
                                i, min,j:entierement
                                debut
                                    ecrire ("entrer la taille du tableau");
                                    lire(n);
                                    ecrire("entrer les valeurs du tableau");
                                    pour (j <- 0 a n-1) faire
                                        lire (T[j]);
                                    finpour
                                    pour (i <- 0 a n-1) faire
                                    min <- i;
                                        pour j <- i+1 a n-1 faire
                                            si (T[j]<T[min]) alors
                                                min <- j
                                            finsi
                                        finpour
                                    si (min != i) alors
                                        echanger(T[i] et T[min])
                                    finsi
                                    finpour
                                fin


                                Nous avons les complexites suivantes :
                                -> Dans le pire des cas , la complexite est de l'ordre de o(n^2)
                                -> Dans le meilleur des cas , la complexite est de l'ordre de o(n)

                                    3 - Tri par insertion O(n^2) :

                                Le Principe est le suivant :
                                chaque element est inserer a sa place correcte parmis les elements deja tries de la liste 
                                (comme dans le jeu des cartes). On constate qu'a la nieme etape le ieme element est insere a son rang (position)
                                parmis les i-1 elements precedements tries. Le mecanisme suivant presente explicitement le Principe.
                                    supposons la liste precedente constitue de 06 elements :

                                        //les premiers elements de la liste sont tries
                                        Liste: 101 115 30 63 47 20
                                        1ere insertion: 30 101 115 | 63 47 20
                                        2eme insertion: 30 63 101 115 | 47 20
                                        3eme insertion: 30 47 63 101 115 | 20
                                        4eme insertion: 20 30 47 63 101 115

                                    son algorithme est le suivant :

                                Algorithme tri_insertion
                                var  T:tableau[o..n] d'entiers
                                var i,j,x :entier
                                debut
                                    ecrire("Entrez la taille du tableau: ");
                                    lire(n);

                                        ecrire("remplissez le tableau");
                                        pour (i <- 0 a n) faire
                                            lire(T[i]);
                                        finpour

                                        pour (i <- 1 a n) faire
                                            x = T[i];
                                            j <- i -1;
                                            tantque (j > 0 et T[j] > x) faire
                                                T[j+1] <- T[j];
                                                j <- j + 1;
                                            fintanque
                                            T[j+1] < x;
                                        finpour
                                fin
                                

                                    4 - Tri fusion (Merge sort) O(n log n):

                                C'est un tri recursif qui repose sur la strategie "diviser pour regner". Le principe est le suivant :

                                    -> diviser le tableau en deux moitie
                                    -> trier recursivement chaque moitie
                                    -> fusionner les deux moitie triee pour obtenir un tableau final "ordonne"

                                    a - Avantages et Inconvenients :
                                        
                                       ** Avantages :

                                   -> le tri par fusion est stable car preserve l'ordre des elements egaux
                                   -> tres performant pour les grands tableaux
                                   -> necessite le meme temps d'execution quelque soit "l'ordre initial"
                                   
                                       ** Inconvenients :

                                    -> necessite une memoire supplementaire (tableau temporaire)
                                    -> plus complexe a implemeter que les tris simples

                                    b - Deroulement de l'algorithme :

                                    -> Si le tableau contient plus d'un element, trouver la position du milieu, puis creer deux sous - tableaux (Gauches et Droites)
                                    -> Appeler recursivement le tri fusion sur chaque sous tableaux
                                    -> Fusionner les deux sous - tableaux Tries en Un Seul 

                                    Exemple : Ecrire le Pseudo-Code correspondant.
                                    

                                    5 - Tri Rapide (Quick Sort) :

                                        5 - 1 - Principe :
                                    
                                        Il repose sur le principe de Pivot (divide le tsbleau en fonction du pivot choisi):

                                    -> Choisir un element du tableau (Pivot)
                                    -> Reorganiser le tableau de sorte que tous les elements inferieurs au Pivot soient a Gauche et ceux superieurs soient a droite
                                    -> Appliquer Recursivement le tri rapide sur chaque sous - tableaux

                                        5 - 2 - Avantages et Inconvenients : 
                                    
                                        ** Avantages :

                                    -> Tres rapide en pratique car tres Rapide que le Tri par Fusion
                                    -> Ne necessite pas de Memoires Supplementaires


                                        ** Inconvenients :

                                    -> Il est Non Stable (car les elements egaux peuvent etre Permutes)
                                    -> La complexite dans le pire des cas peut avoisinner O(n^2) si le Pivot est mal choisi

                                        5 - 3 - Optimisation :

                                    -> Choisir le Pivot comme "Mediane" de trois elements ( * |*|<- Medianne *)
                                    -> Passer a un Tri Simple (Insertion) pour les petites tailles
                                    
                                        5 - 4 - Etapes de l'algorithme :

                                    -> Choisir le Pivot (souvent le Premier ou le Dernier element)
                                    -> Partionner / Rearranger le Tableau autour du Pivot
                                    -> Effectuer la Recursion sur les Sous Tableaux Gauches et Droites

                                    Exemple : Exercice ecrire l'algorithme correspondant
                                
