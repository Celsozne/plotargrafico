package grafico;

import java.awt.*;
import javax.swing.*;

public class seno2 extends JPanel {
    int scale =200;
    int cycles;
    int points;
    double [] seno;
    int [] pts;

    public void setCycles(int cycles) {
        this.cycles = cycles;
        this.points = scale * cycles *2;
        this.seno = new double[points];
        for (int i = 0; i < points; i++) {
            double radianos = (Math.PI/scale)*i;
            this.seno[i] = Math.sin(radianos);
        }
    }

    public void paintcomponent(Graphics g) {
        int maxWidth = getWidth();
        double hstep = (double) maxWidth/ (double) points;
        int maxHeight = getHeight();
        pts = new int[points];
        for (int i = 0; i < points; i++) {
            pts[i] = (int) (seno[i] * maxHeight/2 * .95 + maxWidth/2);
        }
        g.setColor(Color.BLACK);
        for (int i = 0; i < points; i++) {
            int x1 = (int) ((i-1)*hstep);
            int x2 = (int) (i *hstep);
            int y1 = pts[i-1];
            int y2 = pts[i];;
            g.drawLine(x1, y1, x2, y2);
        }
    }

    public static void main(String[] args) {
        JFrame.setDefaultLookAndFeelDecorated(true);
        JFrame frame = new JFrame("Seno");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setBackground(Color.WHITE);
        frame.setSize(500,200);

        seno2 sw = new seno2();
        sw.setCycles(10);
        frame.add(sw);
        frame.setVisible(true);
    }
}
