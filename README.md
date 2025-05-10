# ID_flag_card

Identifica a bandeira de um Cartão de Crédito a Partir do Número do Cartão.

### Explicação do Código:
1. **Importação do Módulo `io`**: Usado para ler a entrada do usuário.
2. **Função `main`**: Ponto de entrada do programa.
3. **Leitura da Entrada**: Solicita ao usuário que digite o número do cartão e lê a entrada.
4. **Validação da Entrada**: Remove espaços em branco e verifica se a entrada contém apenas dígitos. Se não, exibe uma mensagem de erro.
5. **Identificação da Bandeira**: Usa um `match` com padrões para verificar os dígitos iniciais e determinar a bandeira do cartão, incluindo as novas bandeiras e intervalos fornecidos.
6. **Saída**: Exibe a bandeira identificada no formato solicitado.
