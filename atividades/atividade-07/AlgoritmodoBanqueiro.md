# Atividade 07 - Algoritmo do Banqueiro

Feito por João Guilherme Santos de Sousa e Karen Menezes Ribeiro.

Nesta atividade rodamos o algoritmo do Banqueiro visto em aula (Tanenbaum & Bos, cap. 6). O script `bankers_algorithm.py` monta um sistema com 5 processos (P0 a P4) e 3 tipos de recurso (R0, R1, R2) e faz três testes em sequência. Para rodar, basta usar `python3 bankers_algorithm.py`.

O sistema começa com 10, 5 e 7 unidades de cada recurso. A tabela abaixo mostra o que cada processo já tem alocado, o máximo que ele declarou e quanto ainda falta (Need = Max - Alocado):

| Processo | Alocado | Máximo | Need |
|---|---|---|---|
| P0 | [0, 1, 0] | [7, 5, 3] | [7, 4, 3] |
| P1 | [2, 0, 0] | [3, 2, 2] | [1, 2, 2] |
| P2 | [3, 0, 2] | [9, 0, 2] | [6, 0, 0] |
| P3 | [2, 1, 1] | [2, 2, 2] | [0, 1, 1] |
| P4 | [0, 0, 2] | [4, 3, 3] | [4, 3, 1] |

Somando o que já está alocado temos [7, 2, 5], então sobram [3, 3, 2] disponíveis.

## Teste 1: o estado inicial é seguro?

Primeiro o programa verifica se, do jeito que está, existe alguma ordem em que todos os processos conseguem terminar. Ele começa com o vetor Work igual aos disponíveis, [3, 3, 2], e vai procurando um processo cujo Need caiba no Work. Quando acha, considera que esse processo terminou e devolveu o que tinha, somando a alocação dele ao Work.

Na execução, P1 é o primeiro que cabe (precisa de [1, 2, 2]). Depois que ele devolve [2, 0, 0], o Work vai para [5, 3, 2] e aí P3 consegue rodar. Seguindo assim, entram P0, P2 e por último P4, e o Work termina em [10, 5, 7], que é exatamente o total do sistema.

A saída confirma que o estado é seguro, com a sequência P1 -> P3 -> P0 -> P2 -> P4.

<img src="01-estado-inicial.png" width="720" alt="Tela inicial de A.U.F.A. Solar">
<img src="02-estado-seguro.png" width="720" alt="Tela inicial de A.U.F.A. Solar">

## Teste 2: P1 pede [1, 0, 2]

Aqui o P1 faz um pedido e o algoritmo checa três coisas. A primeira é se o pedido não passa do que ele ainda precisa: [1, 0, 2] é menor ou igual a [1, 2, 2], então está ok. A segunda é se tem recurso livre: [1, 0, 2] cabe em [3, 3, 2], então também está ok.

Com isso ele faz uma alocação provisória. Os disponíveis caem para [2, 3, 0], o P1 passa a ter [3, 0, 2] e o Need dele vira [0, 2, 0]. Em seguida roda de novo a verificação de estado seguro com esses valores novos. Ela continua achando a mesma sequência (P1 -> P3 -> P0 -> P2 -> P4), então o pedido é concedido de vez.

Na saída aparece primeiro o aviso de tentativa provisória, depois os passos da verificação e no fim a mensagem de requisição concedida. A tabela impressa logo depois já mostra o P1 com [3, 0, 2] e os disponíveis em [2, 3, 0].

<img src="03-requisicao-p1.png" width="720" alt="Tela inicial de A.U.F.A. Solar">
<img src="04-p1-estado-atualizado.png" width="720" alt="Tela inicial de A.U.F.A. Solar">

## Teste 3: P4 pede [3, 3, 0]

O pedido do P4 está dentro do que ele declarou (o Need dele é [4, 3, 1]), mas depois do teste anterior só sobraram 2 unidades de R0, e ele pediu 3. Como falta recurso, o algoritmo nem tenta a alocação provisória: o P4 simplesmente fica bloqueado esperando algum outro processo liberar R0.

A saída mostra a mensagem de processo bloqueado indicando que pediu 3 de R0 e só havia 2. A tabela impressa depois é igual à anterior, porque nada mudou.

Vale comentar que no código esse teste está descrito como "requisição insegura", mas na prática ele cai no bloqueio por falta de recurso, antes de chegar na verificação de segurança.

<img src="05-p4-bloqueado.png" width="720" alt="Tela inicial de A.U.F.A. Solar">

## Conclusão

Os testes mostram as três situações que o algoritmo trata: conferir se o estado é seguro, conceder um pedido que não coloca o sistema em risco e fazer um processo esperar quando não há recurso suficiente. A ideia é sempre só liberar recursos quando dá para garantir que todo mundo ainda consegue terminar, evitando o deadlock.

