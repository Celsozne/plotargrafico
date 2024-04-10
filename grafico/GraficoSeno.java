package grafico;
import javax.swing.*;
import java.awt.*;

public class GraficoSeno extends JPanel {

    private int xMin, xMax, yMin, yMax;

    public GraficoSeno(int xMin, int xMax, int yMin, int maxY) {
        this.xMin = xMin;
        this.xMax = xMax;
        this.yMin = yMin;
        this.yMax = maxY;
    }

    @Override
    public void paintComponent(Graphics g) {
        super.paintComponent(g);

        // Desenha os eixos
        g.drawLine(0, getHeight() / 2, getWidth(), getHeight() / 2);
        g.drawLine(getWidth() / 2, 0, getWidth() / 2, getHeight());

        // Marcas nos eixos
        for (int i = xMin; i <= xMax; i++) {
            g.drawString(String.valueOf(i), i * getWidth() / (xMax - xMin), getHeight() / 2 + 5);
        }

        // Calcula e plota pontos do seno
        double amplitude = (yMax - yMin) / 2.0;  // Amplitude da onda
        double midline = (yMax + yMin) / 2.0;  // Linha central

        for (int x = xMin; x <= xMax; x++) {
            double y = amplitude * Math.sin((x * Math.PI) / (xMax - xMin)) + midline;
            int yPixel = (int) ((y - yMin) * getHeight() / (yMax - yMin));
            g.drawLine(x * getWidth() / (xMax - xMin), yPixel, x * getWidth() / (xMax - xMin), yPixel);
        }
    }

    public static void main(String[] args) {
        JFrame frame = new JFrame("Gráfico de Seno");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(400, 300);

        GraficoSeno grafico = new GraficoSeno(-5, 5, -2, 2);
        frame.add(grafico);

        frame.setVisible(true);
    }
}