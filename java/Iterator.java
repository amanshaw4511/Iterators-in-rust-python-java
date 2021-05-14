import java.util.NoSuchElementException;
import java.util.stream.StreamSupport;


public class Iterator {
    public static void main(String... args) {
        var num = new Digits(14524);
        // StreamSupport.stream(num.spliterator(), false).forEach(System.out::println);
        var count_digit = StreamSupport.stream(num.spliterator(), false).count();
        System.out.println(count_digit);
        // for (int digit : num) {
        //     System.out.println(digit);
        // }
    }
}

class Digits implements Iterable<Integer> {
    int num;

    Digits(int num) {
        this.num = num;
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
