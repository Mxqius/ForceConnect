using System.Drawing;
using System.Drawing.Drawing2D;
using System.Windows.Forms;

namespace ForceConnect.UserControls
{
    public partial class ServiceControl : UserControl
    {
        public string serviceName;
        public sbyte serviceIndex;

        public ServiceControl(string serviceName, sbyte serviceIndex, Bitmap ServiceImage)
        {
            InitializeComponent();
            // Call a method to round the control
            RoundControl();
            ServiceName = serviceName;
            ServiceIndex = serviceIndex;
            this.ServiceImage = ServiceImage;
        }
        private void RoundControl()
        {
            // Define a GraphicsPath to create a rounded rectangle shape
            GraphicsPath path = new GraphicsPath();
            int radius = 20; // You can adjust the radius as needed

            // Create a rectangle with rounded corners
            path.AddArc(0, 0, radius * 2, radius * 2, 180, 90); // Top-left corner
            path.AddArc(this.Width - radius * 2, 0, radius * 2, radius * 2, 270, 90); // Top-right corner
            path.AddArc(this.Width - radius * 2, this.Height - radius * 2, radius * 2, radius * 2, 0, 90); // Bottom-right corner
            path.AddArc(0, this.Height - radius * 2, radius * 2, radius * 2, 90, 90); // Bottom-left corner
            path.CloseFigure();

            // Set the control's region to the rounded rectangle
            this.Region = new Region(path);
        }
        public string ServiceName
        {
            get
            {
                return label1.Text;
            }
            set
            {
                label1.Text = value;
            }
        }
        public sbyte ServiceIndex
        {
            get
            {
                return serviceIndex;
            }
            set
            {
                serviceIndex = value;
            }
        }
        public Bitmap ServiceImage
        {
            get { return (Bitmap)guna2CirclePictureBox1.Image; }
            set { guna2CirclePictureBox1.Image = value; }
        }
    }
}
