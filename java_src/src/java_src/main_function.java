package java_src;

public class main_function {

	public static void main(String[] args) {
		long start_time = System.currentTimeMillis();
		double number = 0;
		double pi = 0d;
		while(number <= 100000000) {
			pi = pi + Math.pow(-1.0, number) / (2.0 * number + 1.0);
			number = number + 1;
		}
		System.out.println("pi = " + pi * 4);
		long end_time = System.currentTimeMillis();
		System.out.println((end_time - start_time) + "ms");
	}

}
