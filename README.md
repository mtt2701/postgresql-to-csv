# About

This program converts a PostgreSQL dump file to a CSV file. It assumes that the dump file stores its data in a COPY block.

Below is an example of a COPY block in a PostgreSQL dump file (data excerpted from [Spice OSS](https://github.com/spiceai/quickstarts/blob/trunk/federation/postgres/cleaned_sales_data.sql)). Note that it contains 2 lines of data between the first line and the final line.

```sql
COPY public.cleaned_sales_data (order_number, quantity_ordered, price_each, order_line_number, sales, order_date, status, quarter, month, year, product_line, msrp, product_code, customer_name, phone, address_line1, address_line2, city, state, postal_code, country, territory, contact_last_name, contact_first_name, deal_size) FROM stdin;
10107	30	95.7	2	2871	2003-02-24 00:00:00	Shipped	1	2	2003	Motorcycles	95	S10_1678	Land of Toys Inc.	2125557818	897 Long Airport Avenue	\N	NYC	NY	10022	USA	\N	Yu	Kwai	Small
10121	34	81.35	5	2765.9	2003-05-07 00:00:00	Shipped	2	5	2003	Motorcycles	95	S10_1678	Reims Collectables	26.47.1555	59 rue de l'Abbaye	\N	Reims	\N	51100	France	EMEA	Henriot	Paul	Small
\.
```

# Usage

```sh
cd postgres_to_csv
cargo run -- [OPTIONS] <INPUT FILE PATH> <OUTPUT FILE PATH>
```

Options:
- `-a`: Add header row to CSV file, which includes the names of each column.
- `-i <string>`: Change delimiter of input file from default (<kbd>tab</kbd>).
- `-o <string>`: Change delimiter of output file from default (`,`).

Note:
- To pass <kbd>tab</kbd> as a command line argument, type `$'\t'`.
- To pass <kbd>space</kbd> as a command line argument, type `' '`.
