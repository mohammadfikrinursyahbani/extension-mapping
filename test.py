import polars as pl
from function_polars import a_times_b

df = pl.DataFrame(
    {
        "id_responden": [1, 2, 3],
        "col_1": [4, 5, 6],
        "col_2": [4, 4, 4]
    }
)

dict_ = [{'id_responden': 1, 'kolom': 'col_1', 'label': 2}]

result = a_times_b(df, dict_)
print(result)