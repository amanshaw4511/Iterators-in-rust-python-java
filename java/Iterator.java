import java.util.List;
import java.util.NoSuchElementException;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;


public class Iterator {
    public static void main(String... args) {
        // loop through for loop 
        for (int digit : new Digits(14562)) {
            System.out.print(digit + " ");
        }
        println("");

        // collect digits in list
        List<Integer> digit_list = getStream(new Digits(14562)).collect(Collectors.toList());
        println("digit_list: " + digit_list);

        // count digit, max and min of digits
        int digit_count = (int)getStream(new Digits(14562)).count();
        int digit_max = getStream(new Digits(14562)).max((a,b) -> a-b).get();
        int digit_min = getStream(new Digits(14562)).min((a,b) -> a-b).get();
        println("digit_count: " + digit_count);
        println("digit_max: " + digit_max);
        println("digit_min: " + digit_min);

        // sum and product of digits 
        int digit_sum = getStream(new Digits(14562)).reduce(0, (a,x) -> a + x);
        int digit_product = getStream(new Digits(14562)).reduce(1, (a,x) -> a * x);
        println("digit_sum: " + digit_sum);
        println("digit_product: " + digit_product);

        // abs differnce between sum of even digit and sum of odd digit
        int digit_even_sum = getStream(new Digits(14562))
                                .filter(e -> e % 2 == 0)
                                .reduce(0, (a,x) -> a + x);
        int digit_odd_sum = getStream(new Digits(14562))
                                .filter(e -> e % 2 != 0)
                                .reduce(0, (a,x) -> a + x);
        int diff = Math.abs(digit_even_sum - digit_odd_sum);
        println("digit_abs_diff_even_odd: " + diff);

        // number in sorted digit
        int digit_sorted = getStream(new Digits(14562))
                                .sorted()
                                .reduce(0, (a,x) -> (a * 10) + x);
        println("digit_sorted: " + digit_sorted);

    }


    public static Stream<Integer> getStream(Iterable<Integer> iterable) {
        return StreamSupport.stream(iterable.spliterator(), false);
    }

    public static void println(Object obj) {
        System.out.println(obj);
    }
}


class Digits implements Iterable<Integer> {
    private int num;

    Digits(int num) {
        this.num = num;
    }

    public int getNumber() {
        return this.num;
    }

	@Override
	public java.util.Iterator<Integer> iterator() {
		return new java.util.Iterator<Integer>(){

			@Override
			public boolean hasNext() {
                return num > 0 ;
			}

			@Override
			public Integer next() {
                if (!this.hasNext()) throw new NoSuchElementException();
                int digit = num % 10;
                num = num / 10;
				return digit;
			}
		    
		};
	}
}
