describe('It Renders ', () => {
	it('Has default dark theme', () => {
		cy.visit('http://localhost:8000/');
		cy.get('#main').should('have.class', 'dark');
	});

	it('Render site onfo', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=nav-site-info]').click();
		cy.get('[data-cy=site-info]').should('be.visible');
	});

	it('Should switch themes', () => {
		cy.visit('http://localhost:8000/');
		cy.get('[data-cy=theme-toggle]').click();
		cy.get('[data-cy="Light Mode"]').click();
		cy.get('#main').should('have.class', 'light');
	});
});
