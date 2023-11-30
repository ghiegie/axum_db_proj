USE [SampleDatabase]
GO
/****** Object:  Table [dbo].[CustomerTbl]    Script Date: 11/30/2023 5:31:32 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[CustomerTbl](
	[CustomerID] [int] IDENTITY(1,1) NOT NULL,
	[Name] [nvarchar](50) NOT NULL,
	[Addr] [nvarchar](50) NOT NULL,
	[TIN] [nvarchar](50) NOT NULL,
	[TelNo] [nvarchar](50) NOT NULL,
	[Email] [nvarchar](50) NOT NULL,
	[ContPers] [nvarchar](50) NOT NULL,
	[DelivAddr] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_CustomerTbl] PRIMARY KEY CLUSTERED 
(
	[CustomerID] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[ProductTbl]    Script Date: 11/30/2023 5:31:33 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[ProductTbl](
	[ProductID] [int] IDENTITY(1,1) NOT NULL,
	[Product] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_ProductTbl] PRIMARY KEY CLUSTERED 
(
	[ProductID] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[SalesOrderTbl]    Script Date: 11/30/2023 5:31:33 PM ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[SalesOrderTbl](
	[SalesOrderID] [int] IDENTITY(1,1) NOT NULL,
	[CustomerID] [int] NOT NULL,
	[ProductID] [int] NOT NULL,
	[ProdQty] [int] NOT NULL,
	[Status] [nvarchar](50) NOT NULL,
 CONSTRAINT [PK_SalesOrderTbl] PRIMARY KEY CLUSTERED 
(
	[SalesOrderID] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[CustomerTbl]  WITH CHECK ADD  CONSTRAINT [FK_CustomerTbl_CustomerTbl] FOREIGN KEY([CustomerID])
REFERENCES [dbo].[CustomerTbl] ([CustomerID])
GO
ALTER TABLE [dbo].[CustomerTbl] CHECK CONSTRAINT [FK_CustomerTbl_CustomerTbl]
GO
ALTER TABLE [dbo].[ProductTbl]  WITH CHECK ADD  CONSTRAINT [FK_ProductTbl_ProductTbl] FOREIGN KEY([ProductID])
REFERENCES [dbo].[ProductTbl] ([ProductID])
GO
ALTER TABLE [dbo].[ProductTbl] CHECK CONSTRAINT [FK_ProductTbl_ProductTbl]
GO
ALTER TABLE [dbo].[SalesOrderTbl]  WITH CHECK ADD  CONSTRAINT [FK_SalesOrderTbl_CustomerTbl] FOREIGN KEY([CustomerID])
REFERENCES [dbo].[CustomerTbl] ([CustomerID])
GO
ALTER TABLE [dbo].[SalesOrderTbl] CHECK CONSTRAINT [FK_SalesOrderTbl_CustomerTbl]
GO
ALTER TABLE [dbo].[SalesOrderTbl]  WITH CHECK ADD  CONSTRAINT [FK_SalesOrderTbl_ProductTbl] FOREIGN KEY([ProductID])
REFERENCES [dbo].[ProductTbl] ([ProductID])
GO
ALTER TABLE [dbo].[SalesOrderTbl] CHECK CONSTRAINT [FK_SalesOrderTbl_ProductTbl]
GO
