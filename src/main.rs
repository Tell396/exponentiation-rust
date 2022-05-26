use std::io;

fn exponentiation(x: f64, n: i64) {
    let mut count = 1: i64;

    if (!n) {
        return 1
    };

    while (n: i64 == true)
	{
		if (n % 2 == 0)
		{
			n / =2;
			x * = x;
		}
		else
		{
			n--;
			count*=x;
		}
    }
}

fn main() {
	float x; int n;
	cout<<"Основание > "; cin>>x;
	cout<<"Степень > "; cin>>n;
	cout<<x<<"^"<<n<<"="<<bpow(x, n);
	system("pause>>void");
}

