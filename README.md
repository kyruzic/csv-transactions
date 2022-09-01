# Architecture

The program is architected to stream data from a file without loading it all into memory. The only data that is clone is
deposit and withdrawal transactions, this need to be stored on a client for when they are later disputed. We can then read
through the transactions on the client and find the one under dispute.

Only minor changes to how the data is read would be required for this to work in realtime vs a TCP stream, or multiple
concurrent TCP streams.

Fixed numbers or fixed point numbers are used in the place of floating point numbers. I have experience in banking and 
understand the importance of numbers being accurate, floating points cannot give the accuracy that is required when 
dealing with money. Cryptocurrencies follow this same philosophy, where decimal places are only really an illusion that
is added at display time. i.e. ethereum has a field decimals that just puts the decimal place at that value in your balance.

# Safety

The only possible locations for panics is if the input file is not found. I debated about also panicking if there was
an invalid transaction type, but decided instead to just return an error and log that to stderr. Other errors like withdrawals
when the client doesn't have enough funds are simply ignored.

# Testing

All types of transactions are fairly thoroughly tested, additional integration style testing for this application would
need to be made if this were to be hocked into something. The tests you guys will run against that are more what I would 
consider integration testing. Type safety and memory safety are ensured by the compiler so nothing is needed there.

Given more time I would likely write further unit testing for scenarios that I can't think of at the moment, but I 
covered what I consider to be all the fundamental cases.

I have included a sample test file 'input.csv' that I used. 

# Assumptions

- Negative deposits and withdrawals are assumed to be errors and ignored
- If a transaction is disputed and another transaction is received to dispute the same transaction do nothing
- Disputes are valid for withdrawals as well as deposits, for a withdrawal the dispute should increase held
and do nothing to available, resolving the withdrawal increase available and decreases held
- If the transaction type is an invalid string then we panic, this isn't the best way to handle it in a large program
but for this it does the job without adding extra complication. The only other thing we could do is print out that we found
an invalid transaction, but that really does nothing.
- Given the scenario where a user makes two deposits and one withdrawal which is greater than either deposit,
then the user disputes a deposit, available should become a negative number, and while total remains the sum
of available and held.

i.e. this 

| type       | client | tx  | amount |
|------------|--------|-----|--------|
| deposit    | 1      | 1   | 1.0    |
| deposit    | 1      | 2   | 2.0    |
| withdrawal | 1      | 3   | 2.0    |
| dispute    | 1      | 2   |        |

results in

| client | available | held | total | locked |
|--------|-----------|------|-------|--------|
| 1      | -0.5      | 2.0  | 1.5   | false  |

The reasoning for this assumption is it follows the documentation exactly, and seems valid as it prevents the
user from creating money out of nothing. Which they could do if we didn't decrease the available into negatives.

If a chargeback occurs following this then available and total will decrement resulting in a negative available and total