//boucle pour mettre la diagonale a 0 
int A[i][j]
for (i =0 ; i <= n; i++){
    for (j=0 ; j<=n; j++){
        if(i==j){
            A[i][j]=0;
        }
    }
}
scanf("%d", &A[i][j]);

//Alternative plus simple
for (i =0 ; i <= n; i++){
    A[i][i]=0;
}

Exercice 7 : 
1- Matrice d'adjacence : 

          1  2  3  4 
        1 0  1  1  0
        2 1  0  0  1
        3 1  0  0  1
        4 0  1  1  0

2-  1 dans la matrice d'adjacence signifie que les sommets sont adjacents 
(il existe une arête entre eux) et 0 signifie qu'ils ne sont pas adjacents
 (il n'existe pas d arête entre eux).

3- La diagonale vaut 0 (c'est a dire pas de boucles) car un sommet n'est pas adjacent a lui même.